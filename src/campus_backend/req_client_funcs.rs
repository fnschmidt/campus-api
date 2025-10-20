use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use cookie_store::CookieStore;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;
use reqwest_cookie_store::CookieStoreMutex;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use scraper::{Html, Selector, selectable::Selectable};

use crate::{
    constants::CD_CERT_PEM,
    types::{
        CampusDualGrade, CampusDualSignupOption, CampusDualSubGrade, CampusDualVerfahrenOption,
        ExamRegistrationMetadata, GradeResultsTableType, SubGradeMetadata,
    },
};

pub fn get_client_default(retry: bool) -> Result<ClientWithMiddleware> {
    let retries = if retry { 2 } else { 0 };

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(retries);
    Ok(ClientBuilder::new(
        reqwest::Client::builder()
            .add_root_certificate(CD_CERT_PEM.get().unwrap().clone())
            .use_rustls_tls()
            .build()?,
    )
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build())
}

pub fn get_client_with_cd_cookie(retry: bool, j_cookie: String) -> Result<ClientWithMiddleware> {
    let retries = if retry { 2 } else { 0 };

    let cookie: cookie_store::Cookie = serde_json::from_str(&j_cookie)?;
    let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new()));
    {
        let mut store = cookie_store.lock().unwrap();
        store.insert(cookie, &Url::parse("https://campus-dual.de")?)?;
    }

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(retries);
    Ok(ClientBuilder::new(
        reqwest::Client::builder()
            .add_root_certificate(CD_CERT_PEM.get().unwrap().clone())
            .cookie_provider(cookie_store)
            .use_rustls_tls()
            .build()?,
    )
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build())
}

pub fn extract_grades(html_text: String) -> Result<Vec<CampusDualGrade>> {
    lazy_static! {
        static ref IMG_SEL: Selector = Selector::parse("img").unwrap();
        static ref TABLE_SEL: Selector = Selector::parse("#acwork tbody").unwrap();
        static ref NORMAL_MODULE_SEL: Selector = Selector::parse(".child-of-node-0").unwrap();
        static ref TEILPRUEFUNG_SEL: Selector = Selector::parse(".child-of-node-1000").unwrap();
        static ref TD_SEL: Selector = Selector::parse("td").unwrap();
        static ref METADATA_SEL: Selector = Selector::parse("td>div#mscore>a").unwrap();
    };

    let mut grades = Vec::new();

    let document = Html::parse_document(&html_text);
    let table = match document.select(&TABLE_SEL).next() {
        Some(table) => table,
        None => {
            log::warn!("grades table doesnt exist");
            return Ok(vec![]);
        }
    };

    let normal_module_lines = table.select(&NORMAL_MODULE_SEL);
    for line in normal_module_lines {
        let l_id = line
            .value()
            .attr("id")
            .context("CD: grades table line has no ID")?;
        let mut content = line.select(&TD_SEL);
        let table_fields = GradeResultsTableType::from(&mut content);

        let name = table_fields
            .name_el
            .and_then(|el| el.text().next())
            .unwrap_or("Kein Name")
            .to_string();

        let grade = table_fields
            .grade_el
            .and_then(|el| el.text().next())
            .unwrap_or("?")
            .to_string();

        let total_passed = table_fields
            .passed_el
            .and_then(|el| el.select(&IMG_SEL).next())
            .and_then(|img| img.value().attr("src"))
            .map(|src| src.contains("green.png"));

        let credit_points = table_fields
            .ects_el
            .and_then(|el| el.text().next())
            .map(|text| text.trim_start())
            .and_then(|trimmed| trimmed.parse::<i32>().ok())
            .unwrap_or_default();

        let akad_period = table_fields
            .akad_period_el
            .and_then(|el| el.text().next())
            .unwrap_or("nicht vorhanden")
            .to_string();

        let mut subgrades: Vec<CampusDualSubGrade> = Vec::new();
        for grade_subgrade_line in
            table.select(&Selector::parse(&format!(".child-of-{}", l_id)).unwrap())
        {
            let mut content = grade_subgrade_line.select(&TD_SEL);
            let sub_table_fields = GradeResultsTableType::from(&mut content);

            let sub_grade = CampusDualSubGrade {
                name: sub_table_fields
                    .name_el
                    .and_then(|el| el.text().next())
                    .map(|text| text.trim_start())
                    .unwrap_or("kein Name")
                    .to_string(),

                grade: sub_table_fields
                    .grade_el
                    .and_then(|el| el.text().next())
                    .unwrap_or("?")
                    .to_string(),

                passed: sub_table_fields
                    .passed_el
                    .and_then(|el| el.select(&IMG_SEL).next())
                    .and_then(|img| img.value().attr("src"))
                    .map(|src| src.contains("green.png")),

                beurteilung: sub_table_fields
                    .beurteilung_el
                    .and_then(|el| el.text().next())
                    .unwrap_or("01.01.1970")
                    .to_string(),
                bekanntgabe: sub_table_fields
                    .bekanntgabe_el
                    .and_then(|el| el.text().next())
                    .unwrap_or("01.01.1970")
                    .to_string(),
                wiederholung: sub_table_fields
                    .wiederholung_el
                    .and_then(|el| el.text().next())
                    .map(|s| s.to_string()),
                akad_period: sub_table_fields
                    .akad_period_el
                    .and_then(|el| el.text().next())
                    .unwrap_or("nicht vorhanden")
                    .to_string(),
                internal_metadata: grade_subgrade_line.select(&METADATA_SEL).next().and_then(
                    |internal_metadata| {
                        let module = internal_metadata.attr("data-module")?;
                        let peryr = internal_metadata.attr("data-peryr")?;
                        let perid = internal_metadata.attr("data-perid")?;

                        Some(SubGradeMetadata {
                            module: module.to_string(),
                            peryr: peryr.to_string(),
                            perid: perid.to_string(),
                        })
                    },
                ),
            };
            subgrades.push(sub_grade);
        }
        grades.push(CampusDualGrade {
            name,
            grade,
            total_passed,
            credit_points,
            akad_period,
            subgrades,
        });
    }

    // get teilpruefungen
    for teilpruefung_line in table.select(&TEILPRUEFUNG_SEL) {
        let content_selector = &TD_SEL;
        let mut content = teilpruefung_line.select(content_selector);

        let name = content
            .next()
            .and_then(|el| el.text().next())
            .map(|name| name.trim())
            .unwrap_or("Ohne Name");

        let grade = content
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or("?");

        let total_passed = content
            .next()
            .and_then(|el| el.select(&IMG_SEL).next())
            .and_then(|img| img.value().attr("src"))
            .map(|src| src.contains("green.png"));

        let beurteilung = content
            .nth(1)
            .and_then(|el| el.text().next())
            .unwrap_or("01.01.1970")
            .to_string();
        let bekanntgabe = content
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or("01.01.1970")
            .to_string();
        let akad_period = content
            .nth(1)
            .and_then(|el| el.text().next())
            .unwrap_or("nicht vorhanden")
            .to_string();

        let subgrades = vec![CampusDualSubGrade {
            name: name.to_string(),
            grade: grade.to_string(),
            passed: total_passed,
            beurteilung,
            bekanntgabe,
            wiederholung: None,
            akad_period: akad_period.clone(),
            internal_metadata: None,
        }];

        grades.push(CampusDualGrade {
            name: name.to_string(),
            grade: grade.to_string(),
            total_passed,
            credit_points: 0,
            akad_period,
            subgrades,
        });
    }

    grades.sort_by(|grade_a, grade_b| {
        get_newest_subgrade_date(grade_b).cmp(&get_newest_subgrade_date(grade_a))
    });

    Ok(grades)
}

fn get_newest_subgrade_date(grade: &CampusDualGrade) -> NaiveDate {
    let newest_subgrade = grade.subgrades.iter().max_by(|a, b| {
        let date_a = NaiveDate::parse_from_str(&a.bekanntgabe, "%d.%m.%Y").unwrap_or_default();
        let date_b = NaiveDate::parse_from_str(&b.bekanntgabe, "%d.%m.%Y").unwrap_or_default();

        date_a.cmp(&date_b)
    });

    if let Some(subgrade) = newest_subgrade {
        NaiveDate::parse_from_str(&subgrade.bekanntgabe, "%d.%m.%Y").unwrap_or_default()
    } else {
        NaiveDate::default()
    }
}

pub async fn extract_exam_signup_options(html_text: String) -> Result<Vec<CampusDualSignupOption>> {
    lazy_static! {
        static ref IMG_SEL: Selector = Selector::parse("img").unwrap();
        static ref TABLE_SEL: Selector = Selector::parse("#expproc tbody").unwrap();
        static ref NORMAL_LINE_SEL: Selector = Selector::parse(".child-of-node-0").unwrap();
        static ref TD_SEL: Selector = Selector::parse("td").unwrap();
        static ref METADATA_SEL: Selector = Selector::parse("td>a.booking").unwrap();
    };

    let mut signup_options = Vec::new();

    let document = Html::parse_document(&html_text);
    let table = match document.select(&TABLE_SEL).next() {
        Some(table) => table,
        None => return Ok(vec![]),
    };

    let top_level_lines = table.select(&NORMAL_LINE_SEL);
    for line in top_level_lines {
        let l_id = match line.value().attr("id") {
            None => continue,
            Some(l_id) => l_id,
        };

        let mut content = line.select(&TD_SEL);

        let name = content
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or("Kein Name")
            .to_string();
        let verfahren = content
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or_default()
            .to_string();
        let pruefart = content
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or_default()
            .to_string();

        let subline_selector = &Selector::parse(&format!(".child-of-{l_id}")).unwrap();
        let mut sublines = table.select(subline_selector);

        let main_subline = match sublines.next() {
            Some(line) => line,
            None => continue,
        };

        let internal_metadata = main_subline
            .select(&METADATA_SEL)
            .next()
            .and_then(|meta_el| {
                Some(ExamRegistrationMetadata {
                    assessment: meta_el.value().attr("data-evob_objid")?.to_string(),
                    peryr: meta_el.value().attr("data-peryr")?.to_string(),
                    perid: meta_el.value().attr("data-perid")?.to_string(),
                    offerno: meta_el.value().attr("data-offerno")?.to_string(),
                })
            });

        let status_icon_url = main_subline
            .select(&IMG_SEL)
            .next()
            .and_then(|img| img.value().attr("src"))
            .unwrap_or_default();
        let status = match status_icon_url {
            "/images/missed.png" => "🚫",
            "/images/yellow.png" => "📝",
            "/images/exclamation.jpg" => "⚠️",
            _ => "⁉️",
        }
        .to_string();

        // my shoddy code demands that the iterator is over owned values and not references,
        // else the iterator doesn't consume the values and causes wrapping after the first None
        let mut main_subline_texts = main_subline.text().collect::<Vec<_>>().into_iter();

        if main_subline_texts.len() == 0 {
            signup_options.push(CampusDualSignupOption {
                name,
                verfahren,
                pruefart,
                status,
                signup_information: "Daten konnten nicht extrahiert werden".to_string(),
                exam_date: None,
                exam_time: None,
                exam_room: None,
                warning_message: None,
                signup_until: None,
                internal_metadata: None,
            });

            continue;
        }

        let signup_information_messy = main_subline_texts
            .next()
            .map(|text| text.trim_start())
            .unwrap_or_default();
        let signup_information =
            if let Some(stripped) = signup_information_messy.strip_suffix(", Prüfungstermin: ") {
                stripped
            } else {
                signup_information_messy
            }
            .to_string();

        let exam_date = main_subline_texts.next().map(|el| el.to_string());
        let exam_time = main_subline_texts.nth(1).map(|el| el.to_string());
        let exam_room = main_subline_texts
            .next()
            .map(|el| el.strip_prefix(", ").unwrap_or(el).to_string());

        let warning_message = sublines.next().map(|second_subline| {
            second_subline
                .text()
                .fold(String::new(), |a, b| a + b)
                .trim_start()
                // campusdual html is fucked in case of no time, so replace that but leave 1 space
                .replace("   :  ", "")
                .to_string()
        });
        lazy_static! {
            static ref RE: Regex = Regex::new(r"bis (\d{2}\.\d{2}\.\d{4})").unwrap();
        }
        let signup_until = warning_message.as_ref().and_then(|msg| {
            RE.captures(msg)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        });

        signup_options.push(CampusDualSignupOption {
            name,
            verfahren,
            pruefart,
            status,
            signup_information,
            exam_date,
            exam_time,
            exam_room,
            warning_message,
            signup_until,
            internal_metadata,
        });
    }

    Ok(signup_options)
}

pub async fn extract_exam_verfahren_options(
    html_text: String,
) -> Result<Vec<CampusDualVerfahrenOption>> {
    lazy_static! {
        static ref IMG_SEL: Selector = Selector::parse("img").unwrap();
        static ref TABLE_SEL: Selector = Selector::parse("#exopen tbody").unwrap();
        static ref NORMAL_LINE_SEL: Selector = Selector::parse(".child-of-node-0").unwrap();
        static ref TD_SEL: Selector = Selector::parse("td").unwrap();
        static ref METADATA_SEL: Selector = Selector::parse("td>a.booking").unwrap();
    };

    let mut signup_options = Vec::new();

    let document = Html::parse_document(&html_text);
    let table = match document.select(&TABLE_SEL).next() {
        Some(table) => table,
        None => {
            log::warn!("extract_exam_verfahren: no table");
            return Ok(vec![]);
        }
    };

    let top_level_lines = table.select(&NORMAL_LINE_SEL);
    for line in top_level_lines {
        let l_id = match line.value().attr("id") {
            Some(l_id) => l_id,
            None => continue,
        };

        let mut content = line.select(&TD_SEL);

        let name = content
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or("Kein Name")
            .to_string();
        let verfahren = content
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or("Kein Name")
            .to_string();
        let pruefart = content
            .next()
            .and_then(|el| el.text().next())
            .unwrap()
            .to_string();

        let subline_selector = &Selector::parse(&format!(".child-of-{l_id}")).unwrap();
        let mut sublines = table.select(subline_selector);
        let main_subline = sublines.next().unwrap();

        let internal_metadata =
            main_subline
                .select(&METADATA_SEL)
                .next()
                .map(|meta_el| ExamRegistrationMetadata {
                    assessment: meta_el.value().attr("data-evob_objid").unwrap().to_string(),
                    peryr: meta_el.value().attr("data-peryr").unwrap().to_string(),
                    perid: meta_el.value().attr("data-perid").unwrap().to_string(),
                    offerno: meta_el.value().attr("data-offerno").unwrap().to_string(),
                });

        let status_icon_url = main_subline
            .select(&IMG_SEL)
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap();
        let status = match status_icon_url {
            "/images/missed.png" => "🚫",
            "/images/yellow.png" => "📝",
            "/images/exclamation.jpg" => "⚠️",
            _ => "⁉️",
        }
        .to_string();

        // my shoddy code demands that the iterator is over owned values and not references,
        // else the iterator doesn't consume the values and causes wrapping after the first None
        let mut main_subline_texts = main_subline.text().collect::<Vec<_>>().into_iter();

        if main_subline_texts.len() == 0 {
            signup_options.push(CampusDualVerfahrenOption {
                name,
                verfahren,
                pruefart,
                status,
                signup_information: "Daten konnten nicht extrahiert werden".to_string(),
                exam_date: None,
                exam_time: None,
                exam_room: None,
                warning_message: None,
                signoff_until: None,
                internal_metadata: None,
            });

            continue;
        }

        let signup_information_messy = main_subline_texts.next().unwrap().trim_start();
        let signup_information = if let Some(stripped) = signup_information_messy
            .split_once("Prüfungstermin")
            .map(|split| split.0.replace(", ", ""))
        {
            stripped
        } else {
            signup_information_messy.to_string()
        };

        let exam_date = main_subline_texts.next().map(|el| el.to_string());
        let exam_time = main_subline_texts.nth(1).map(|el| el.to_string());
        let exam_room = main_subline_texts
            .next()
            .map(|el| el.strip_prefix(", ").unwrap_or(el).to_string());

        let warning_message = sublines.next().map(|second_subline| {
            second_subline
                .text()
                .fold(String::new(), |a, b| a + b)
                .trim_start()
                // campusdual html is fucked in case of no time, so replace that but leave 1 space
                .replace("   :  ", "")
                .to_string()
        });
        lazy_static! {
            static ref RE: Regex = Regex::new(r"bis zum (\d{2}\.\d{2}\.\d{4})").unwrap();
        }
        let signoff_until = warning_message.as_ref().and_then(|msg| {
            RE.captures(msg)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        });

        signup_options.push(CampusDualVerfahrenOption {
            name,
            verfahren,
            pruefart,
            status,
            signup_information,
            exam_date,
            exam_time,
            exam_room,
            warning_message,
            signoff_until,
            internal_metadata,
        });
    }

    Ok(signup_options)
}
