use axum::{Extension, Json, extract::State};
use chrono::{DateTime, Duration, Utc};
use fnv::FnvHasher;
use tokio::time::{Duration as TokioDuration, sleep};

use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::{
    color_stuff::hex_to_luminance,
    types::{
        CampusDualGrade, CampusDualSignupOption, CampusDualVerfahrenOption, CampusReminders,
        CampusTimelineEvent, CdAuthData, CdExamDetails, CdExamStats, ExamRegistrationMetadata,
        ExportTimelineEvent, ExportTimelineEvents, GradeStatsAllStudents, LoginResponse,
        ResponseError, SimulatedState, StundenplanItem, SubGradeMetadata,
    },
};

pub async fn sleep_some() {
    let sleep_time = rand::random_range(200..800);
    sleep(TokioDuration::from_millis(sleep_time)).await;
}

pub async fn get_grades(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualGrade>>, ResponseError> {
    sleep_some().await;
    let grades_json = r#"[
    {
        "name": "Integrierte Informationssysteme (5CS-ERPS-CS)",
        "grade": "1,3",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2023/2024",
        "subgrades": [
            {
                "name": "P Integrierte Informationssysteme (C) (5CS-ERPS-00)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "17.07.2024",
                "bekanntgabe": "18.07.2024",
                "wiederholung": null,
                "akad_period": "SS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-ERPS-00",
                    "peryr": "2024",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Recht (5CS-RECHT-CS)",
        "grade": "1,8",
        "total_passed": true,
        "credit_points": 4,
        "akad_period": "WS 2021/2022",
        "subgrades": [
            {
                "name": "P Recht (K) (5CS-RECHT-00)",
                "grade": "1,8",
                "passed": true,
                "beurteilung": "22.03.2024",
                "bekanntgabe": "22.05.2024",
                "wiederholung": null,
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-RECHT-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Data Science (5CS-DASC-CS)",
        "grade": "1,0",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2023/2024",
        "subgrades": [
            {
                "name": "P Data Science (SE) (5CS-DASC-00)",
                "grade": "1,0",
                "passed": true,
                "beurteilung": "24.03.2024",
                "bekanntgabe": "29.04.2024",
                "wiederholung": null,
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-DASC-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Bildbearb., Comp.grafik u. Comp.animat. (5CS-CGAN-CS)",
        "grade": "1,8",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2021/2022",
        "subgrades": [
            {
                "name": "P Bildbearb., Comp.grafik u. C.anim. (K) (5CS-CGAN-00)",
                "grade": "1,8",
                "passed": true,
                "beurteilung": "14.03.2024",
                "bekanntgabe": "25.04.2024",
                "wiederholung": null,
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-CGAN-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Programmierung in C/C++ (5CS-CPP-CS)",
        "grade": "1,3",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2023/2024",
        "subgrades": [
            {
                "name": "P Programmierung in C/C++ (K) (5CS-CPP-00)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "20.03.2024",
                "bekanntgabe": "02.04.2024",
                "wiederholung": null,
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-CPP-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Softwareprojekt (5CS-SOPR-CS)",
        "grade": "1,0",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2021/2022",
        "subgrades": [
            {
                "name": "P Softwareprojekt (P) (5CS-SOPR-00)",
                "grade": "1,0",
                "passed": true,
                "beurteilung": "19.03.2024",
                "bekanntgabe": "26.03.2024",
                "wiederholung": null,
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-SOPR-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "BWL 2: Rechnungsw. und Investition (5CS-BWLRI-CS)",
        "grade": "3,3",
        "total_passed": true,
        "credit_points": 4,
        "akad_period": "SS 2020/2021",
        "subgrades": [
            {
                "name": "P BWL 2: Rechnungsw. und Investition (K) (5CS-BWLRI-00)",
                "grade": "5,0",
                "passed": false,
                "beurteilung": "18.09.2023",
                "bekanntgabe": "02.10.2023",
                "wiederholung": null,
                "akad_period": "SS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-BWLRI-00",
                    "peryr": "2023",
                    "perid": "002"
                }
            },
            {
                "name": "P BWL 2: Rechnungsw. und Investition (K) (5CS-BWLRI-00)",
                "grade": "3,3",
                "passed": true,
                "beurteilung": "15.02.2024",
                "bekanntgabe": "27.02.2024",
                "wiederholung": "WP1",
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-BWLRI-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Stochastik (5CS-MA3ST-CS)",
        "grade": "3,0",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2020/2021",
        "subgrades": [
            {
                "name": "P Stochastik (K) (5CS-MA3ST-00)",
                "grade": "5,0",
                "passed": false,
                "beurteilung": "11.05.2023",
                "bekanntgabe": "13.06.2023",
                "wiederholung": null,
                "akad_period": "SS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-MA3ST-00",
                    "peryr": "2023",
                    "perid": "002"
                }
            },
            {
                "name": "P Stochastik (K) (5CS-MA3ST-00)",
                "grade": "3,0",
                "passed": true,
                "beurteilung": "12.12.2023",
                "bekanntgabe": "19.12.2023",
                "wiederholung": "WP1",
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-MA3ST-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Eigenständige Projektarbeit (5CS-PT5-CS)",
        "grade": "1,0",
        "total_passed": true,
        "credit_points": 6,
        "akad_period": "WS 2021/2022",
        "subgrades": [
            {
                "name": "P Eigenständige Projektarbeit (M) (5CS-PT5-00)",
                "grade": "1,0",
                "passed": true,
                "beurteilung": "12.12.2023",
                "bekanntgabe": "18.12.2023",
                "wiederholung": null,
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-PT5-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Computernetzw. u. Drahtlose Kommunik. (5CS-CNWC-CS)",
        "grade": "2,5",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2020/2021",
        "subgrades": [
            {
                "name": "P Computernetzw. u. Drahtl. Kommun. (K) (5CS-CNWC-00)",
                "grade": "2,5",
                "passed": true,
                "beurteilung": "22.09.2023",
                "bekanntgabe": "15.11.2023",
                "wiederholung": null,
                "akad_period": "SS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-CNWC-00",
                    "peryr": "2023",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Software Engineering (5CS-PT4-CS)",
        "grade": "1,3",
        "total_passed": true,
        "credit_points": 6,
        "akad_period": "SS 2020/2021",
        "subgrades": [
            {
                "name": "P Software Engineering (PR) (5CS-PT4-00)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "30.06.2023",
                "bekanntgabe": "24.10.2023",
                "wiederholung": null,
                "akad_period": "SS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-PT4-00",
                    "peryr": "2023",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Softwaretechnik und Projektmanagement (5CS-SEPM-CS)",
        "grade": "3,1",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2020/2021",
        "subgrades": [
            {
                "name": "P Softwaretechnik u. Projektmanag. (C) (5CS-SEPM-00)",
                "grade": "3,1",
                "passed": true,
                "beurteilung": "11.09.2023",
                "bekanntgabe": "18.10.2023",
                "wiederholung": null,
                "akad_period": "SS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-SEPM-00",
                    "peryr": "2023",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Numerik (5CS-MA4NU-CS)",
        "grade": "1,0",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2020/2021",
        "subgrades": [
            {
                "name": "P Numerik (SE) (5CS-MA4NU-00)",
                "grade": "1,0",
                "passed": true,
                "beurteilung": "24.09.2023",
                "bekanntgabe": "11.10.2023",
                "wiederholung": null,
                "akad_period": "SS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-MA4NU-00",
                    "peryr": "2023",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Datenschutz und Kryptographie (5CS-DSKRY-CS)",
        "grade": "2,1",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2020/2021",
        "subgrades": [
            {
                "name": "P Datenschutz und Kryptographie (K) (5CS-DSKRY-00)",
                "grade": "2,1",
                "passed": true,
                "beurteilung": "15.09.2023",
                "bekanntgabe": "04.10.2023",
                "wiederholung": null,
                "akad_period": "SS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-DSKRY-00",
                    "peryr": "2023",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Betriebssysteme (5CS-OPSY-CS)",
        "grade": "1,6",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2020/2021",
        "subgrades": [
            {
                "name": "P Betriebssysteme (K) (5CS-OPSY-00)",
                "grade": "1,6",
                "passed": true,
                "beurteilung": "14.03.2023",
                "bekanntgabe": "16.05.2023",
                "wiederholung": null,
                "akad_period": "WS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-OPSY-00",
                    "peryr": "2023",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Algorithmen und Datenstrukturen (5CS-TI2AD-CS)",
        "grade": "1,0",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2020/2021",
        "subgrades": [
            {
                "name": "P Algorithmen und Datenstrukturen (SE) (5CS-TI2AD-00)",
                "grade": "1,0",
                "passed": true,
                "beurteilung": "26.03.2023",
                "bekanntgabe": "11.04.2023",
                "wiederholung": null,
                "akad_period": "WS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-TI2AD-00",
                    "peryr": "2023",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Nutzerinteraktion u. relationale Datenb. (5CS-UIDB-CS)",
        "grade": "2,2",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2020/2021",
        "subgrades": [
            {
                "name": "P Nutzerinterakt. u. relation. DB (C) (5CS-UIDB-00)",
                "grade": "N",
                "passed": null,
                "beurteilung": "21.03.2023",
                "bekanntgabe": "11.04.2023",
                "wiederholung": null,
                "akad_period": "WS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-UIDB-00",
                    "peryr": "2023",
                    "perid": "001"
                }
            },
            {
                "name": "P Nutzerinterakt. u. relation. DB (C) (5CS-UIDB-00)",
                "grade": "2,2",
                "passed": true,
                "beurteilung": "14.12.2023",
                "bekanntgabe": "09.02.2024",
                "wiederholung": "NP",
                "akad_period": "WS 2023/2024",
                "internal_metadata": {
                    "module": "5CS-UIDB-00",
                    "peryr": "2024",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Fachenglisch und Kommunikationstechniken (5CS-ENG2F-CS)",
        "grade": "1,3",
        "total_passed": true,
        "credit_points": 4,
        "akad_period": "WS 2020/2021",
        "subgrades": [
            {
                "name": "P Fachenglisch u. Kommunikationst. (M) (5CS-ENG2F-00)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "09.03.2023",
                "bekanntgabe": "17.03.2023",
                "wiederholung": null,
                "akad_period": "WS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-ENG2F-00",
                    "peryr": "2023",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Digitaltechnik und Rechnerarchitektur (5CS-DTCA-CS)",
        "grade": "3,3",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2019/2020",
        "subgrades": [
            {
                "name": "P Digitaltechnik und Rechnerarchit. (K) (5CS-DTCA-00)",
                "grade": "3,3",
                "passed": true,
                "beurteilung": "21.09.2022",
                "bekanntgabe": "20.12.2022",
                "wiederholung": null,
                "akad_period": "SS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-DTCA-00",
                    "peryr": "2022",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Python (5CS-PYTHN-CS)",
        "grade": "2,0",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2021/2022",
        "subgrades": [
            {
                "name": "P Python (C) (5CS-PYTHN-00)",
                "grade": "2,0",
                "passed": true,
                "beurteilung": "23.09.2022",
                "bekanntgabe": "14.12.2022",
                "wiederholung": null,
                "akad_period": "SS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-PYTHN-00",
                    "peryr": "2022",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Arbeitsorganisation im Team (5CS-PT3-CS)",
        "grade": "1,3",
        "total_passed": true,
        "credit_points": 6,
        "akad_period": "WS 2020/2021",
        "subgrades": [
            {
                "name": "P Arbeitsorganisation im Team (M) (5CS-PT3-00)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "05.12.2022",
                "bekanntgabe": "08.12.2022",
                "wiederholung": null,
                "akad_period": "WS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-PT3-00",
                    "peryr": "2023",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Betriebssysteme und Netzwerke (5CS-PT2-CS)",
        "grade": "1,7",
        "total_passed": true,
        "credit_points": 6,
        "akad_period": "SS 2019/2020",
        "subgrades": [
            {
                "name": "P Betriebssysteme und Netzwerke (PR) (5CS-PT2-00)",
                "grade": "1,7",
                "passed": true,
                "beurteilung": "01.07.2022",
                "bekanntgabe": "30.11.2022",
                "wiederholung": null,
                "akad_period": "SS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-PT2-00",
                    "peryr": "2022",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Datenver. u. Datenbankzugriffssprachen (5CS-DPDL-CS)",
        "grade": "1,2",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2021/2022",
        "subgrades": [
            {
                "name": "P Datenver. u.Datenbankzugriffsspr. (C) (5CS-DPDL-00)",
                "grade": "1,2",
                "passed": true,
                "beurteilung": "16.09.2022",
                "bekanntgabe": "10.11.2022",
                "wiederholung": null,
                "akad_period": "SS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-DPDL-00",
                    "peryr": "2022",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Analysis (5CS-MA2AN-CS)",
        "grade": "4,0",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "SS 2019/2020",
        "subgrades": [
            {
                "name": "P Analysis (K) (5CS-MA2AN-00)",
                "grade": "5,0",
                "passed": false,
                "beurteilung": "19.09.2022",
                "bekanntgabe": "21.10.2022",
                "wiederholung": null,
                "akad_period": "SS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-MA2AN-00",
                    "peryr": "2022",
                    "perid": "002"
                }
            },
            {
                "name": "P Analysis (K) (5CS-MA2AN-00)",
                "grade": "4,0",
                "passed": true,
                "beurteilung": "22.02.2023",
                "bekanntgabe": "02.03.2023",
                "wiederholung": "WP1",
                "akad_period": "WS 2022/2023",
                "internal_metadata": {
                    "module": "5CS-MA2AN-00",
                    "peryr": "2023",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Wirtschaftsenglisch und Kommunikation (5CS-ENG1W-CS)",
        "grade": "1,3",
        "total_passed": true,
        "credit_points": 4,
        "akad_period": "SS 2019/2020",
        "subgrades": [
            {
                "name": "P Wirtschaftsenglisch u. Kommunikat. (K) (5CS-ENG1W-00)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "14.09.2022",
                "bekanntgabe": "28.09.2022",
                "wiederholung": null,
                "akad_period": "SS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-ENG1W-00",
                    "peryr": "2022",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "BWL 1: Personal und Organisation (5CS-BWLPO-CS)",
        "grade": "2,7",
        "total_passed": true,
        "credit_points": 4,
        "akad_period": "WS 2019/2020",
        "subgrades": [
            {
                "name": "P BWL 1: Personal und Organisation (K) (5CS-BWLPO-00)",
                "grade": "5,0",
                "passed": false,
                "beurteilung": "16.12.2019",
                "bekanntgabe": "07.02.2020",
                "wiederholung": null,
                "akad_period": "WS 2019/2020",
                "internal_metadata": {
                    "module": "5CS-BWLPO-00",
                    "peryr": "2020",
                    "perid": "001"
                }
            },
            {
                "name": "P BWL 1: Personal und Organisation (K) (5CS-BWLPO-00)",
                "grade": "2,7",
                "passed": true,
                "beurteilung": "22.03.2022",
                "bekanntgabe": "08.06.2022",
                "wiederholung": "WP1",
                "akad_period": "WS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-BWLPO-00",
                    "peryr": "2022",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Automaten und formale Sprachen (5CS-TI1AS-CS)",
        "grade": "1,3",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2019/2020",
        "subgrades": [
            {
                "name": "P Automaten und formale Sprachen (K) (5CS-TI1AS-00)",
                "grade": "5,0",
                "passed": false,
                "beurteilung": "12.12.2019",
                "bekanntgabe": "12.02.2020",
                "wiederholung": null,
                "akad_period": "WS 2019/2020",
                "internal_metadata": {
                    "module": "5CS-TI1AS-00",
                    "peryr": "2020",
                    "perid": "001"
                }
            },
            {
                "name": "P Automaten und formale Sprachen (K) (5CS-TI1AS-00)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "16.03.2022",
                "bekanntgabe": "25.04.2022",
                "wiederholung": "WP1",
                "akad_period": "WS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-TI1AS-00",
                    "peryr": "2022",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Lineare Algebra (5CS-MA1LA-CS)",
        "grade": "2,7",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2019/2020",
        "subgrades": [
            {
                "name": "P Lineare Algebra (K) (5CS-MA1LA-00)",
                "grade": "5,0",
                "passed": false,
                "beurteilung": "18.12.2019",
                "bekanntgabe": "08.01.2020",
                "wiederholung": null,
                "akad_period": "WS 2019/2020",
                "internal_metadata": {
                    "module": "5CS-MA1LA-00",
                    "peryr": "2020",
                    "perid": "001"
                }
            },
            {
                "name": "P Lineare Algebra (K) (5CS-MA1LA-00)",
                "grade": "2,7",
                "passed": true,
                "beurteilung": "18.03.2022",
                "bekanntgabe": "23.03.2022",
                "wiederholung": "WP1",
                "akad_period": "WS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-MA1LA-00",
                    "peryr": "2022",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "IT-Prozesse des Unternehmens (5CS-PT1-CS)",
        "grade": "2,3",
        "total_passed": true,
        "credit_points": 6,
        "akad_period": "WS 2019/2020",
        "subgrades": [
            {
                "name": "P IT-Prozesse des Unternehmens (P) (5CS-PT1-00)",
                "grade": "2,3",
                "passed": true,
                "beurteilung": "15.02.2022",
                "bekanntgabe": "18.02.2022",
                "wiederholung": null,
                "akad_period": "WS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-PT1-00",
                    "peryr": "2022",
                    "perid": "001"
                }
            }
        ]
    },
    {
        "name": "Grdl. d. Elektrotechn. u.Halbleiterelek. (5CS-ETHLE-CS)",
        "grade": "1,3",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2019/2020",
        "subgrades": [
            {
                "name": "P Grdl. d. Elektrot. u.Halbleiterel. (K) (5CS-ETHLE-00)",
                "grade": "5,0",
                "passed": false,
                "beurteilung": "20.12.2019",
                "bekanntgabe": "26.02.2020",
                "wiederholung": null,
                "akad_period": "WS 2019/2020",
                "internal_metadata": {
                    "module": "5CS-ETHLE-00",
                    "peryr": "2020",
                    "perid": "001"
                }
            },
            {
                "name": "P Grdl. d. Elektrot. u.Halbleiterel. (K) (5CS-ETHLE-00)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "11.08.2022",
                "bekanntgabe": "05.12.2022",
                "wiederholung": "WP1",
                "akad_period": "SS 2021/2022",
                "internal_metadata": {
                    "module": "5CS-ETHLE-00",
                    "peryr": "2022",
                    "perid": "002"
                }
            }
        ]
    },
    {
        "name": "Rechnerprogr. und Web-Technologien (5CS-CPWT-CS)",
        "grade": "3,4",
        "total_passed": true,
        "credit_points": 5,
        "akad_period": "WS 2019/2020",
        "subgrades": [
            {
                "name": "P2 Rechnerprogr. u. Web-Technologien (P) (5CS-CPWT-02)",
                "grade": "1,3",
                "passed": true,
                "beurteilung": "22.10.2019",
                "bekanntgabe": "05.12.2019",
                "wiederholung": null,
                "akad_period": "WS 2019/2020",
                "internal_metadata": {
                    "module": "5CS-CPWT-02",
                    "peryr": "2020",
                    "perid": "001"
                }
            },
            {
                "name": "P1 Rechnerprogr. u. Web-Technologien (P) (5CS-CPWT-01)",
                "grade": "1,0",
                "passed": true,
                "beurteilung": "25.10.2019",
                "bekanntgabe": "03.12.2019",
                "wiederholung": null,
                "akad_period": "WS 2019/2020",
                "internal_metadata": {
                    "module": "5CS-CPWT-01",
                    "peryr": "2020",
                    "perid": "001"
                }
            },
            {
                "name": "P3 Rechnerprogr. u. Web-Technologien (C) (5CS-CPWT-03)",
                "grade": "5,0",
                "passed": false,
                "beurteilung": "10.12.2019",
                "bekanntgabe": "08.01.2020",
                "wiederholung": null,
                "akad_period": "WS 2019/2020",
                "internal_metadata": {
                    "module": "5CS-CPWT-03",
                    "peryr": "2020",
                    "perid": "001"
                }
            }
        ]
    }
]"#;

    let grades: Vec<CampusDualGrade> = serde_json::from_str(grades_json).unwrap();

    Ok(Json(grades))
}

pub async fn get_gradestats(
    Extension(_): Extension<CdAuthData>,
    Json(_): Json<SubGradeMetadata>,
) -> Result<Json<GradeStatsAllStudents>, ResponseError> {
    sleep_some().await;
    let all_stats = GradeStatsAllStudents {
        one: 3,
        two: 8,
        three: 8,
        four: 4,
        ronmodus: 3,
    };

    Ok(Json(all_stats))
}

pub async fn check_revive_session(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Option<LoginResponse>>, ResponseError> {
    println!("checking session...");

    Ok(Json(None))
}

pub async fn get_examsignup(
    State(state): State<Arc<SimulatedState>>,
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualSignupOption>>, ResponseError> {
    let stds_signup_state = *state.stds_signup_states.lock().unwrap();

    sleep_some().await;
    let mut signup_options = vec![];
    if !stds_signup_state.0 {
        signup_options.push(CampusDualSignupOption {
            name: "P1 Servers. Techn. u. vert. Systeme (SE) (5CS-STDS-01)".to_string(),
            verfahren: "SEP".to_string(),
            pruefart: "Sommersemester".to_string(),
            status: "📝".to_string(),
            signup_information: "Prüfungsart: MPMA".to_string(),
            exam_date: Some("32.12.2024".to_string()),
            exam_time: Some("12:34".to_string()),
            exam_room: Some("Ort: 5SR 104".to_string()),
            warning_message: Some("Anmeldung ist nur noch bis morgen möglich.".to_string()),
            signup_until: Some("31.12.2024".to_string()),
            internal_metadata: Some(ExamRegistrationMetadata {
                assessment: "fake1".to_string(),
                peryr: "".to_string(),
                perid: "".to_string(),
                offerno: "".to_string(),
            }),
        });
    };
    if !stds_signup_state.1 {
        signup_options.push(CampusDualSignupOption {
            name: "P1 Servers. Techn. u. vert. Systeme (SE) (5CS-STDS-02)".to_string(),
            verfahren: "SEP".to_string(),
            pruefart: "Sommersemester".to_string(),
            status: "📝".to_string(),
            signup_information: "Prüfungsart: MPMA".to_string(),
            exam_date: Some("33.12.2024".to_string()),
            exam_time: Some("12:34".to_string()),
            exam_room: Some("Ort: 5SR 104".to_string()),
            warning_message: Some("Anmeldung ist nur noch bis morgen möglich.".to_string()),
            signup_until: Some("31.12.2024".to_string()),
            internal_metadata: Some(ExamRegistrationMetadata {
                assessment: "fake2".to_string(),
                peryr: "".to_string(),
                perid: "".to_string(),
                offerno: "".to_string(),
            }),
        })
    };

    signup_options.push(CampusDualSignupOption {
        name: "P Bachelorarbeit SG Informatik (5CS-BACS-00)".to_string(),
        verfahren: "SEP".to_string(),
        pruefart: "Sommersemester".to_string(),
        status: "🚫".to_string(),
        signup_information: "Prüfungsart: BATH".to_string(),
        exam_date: Some("32.12.2024".to_string()),
        exam_time: Some("12:34".to_string()),
        exam_room: Some("Ort: 5SR 104".to_string()),
        warning_message: Some("Anmeldung war nur noch bis gestern möglich.".to_string()),
        signup_until: Some("31.12.2024".to_string()),
        internal_metadata: None,
    });

    Ok(Json(signup_options))
}

pub async fn post_registerexam(
    State(state): State<Arc<SimulatedState>>,
    Extension(_): Extension<CdAuthData>,
    Json(meta): Json<ExamRegistrationMetadata>,
) -> Result<String, ResponseError> {
    sleep_some().await;
    let mut lock = state.stds_signup_states.lock().unwrap();
    match meta.assessment.as_str() {
        "fake1" => lock.0 = true,
        "fake2" => lock.1 = true,
        _ => unreachable!(),
    };
    Ok("{}".to_string())
}

pub async fn get_examdetails(
    Extension(_): Extension<CdAuthData>,
    Json(_): Json<ExamRegistrationMetadata>,
) -> Result<Json<CdExamDetails>, ResponseError> {
    sleep_some().await;
    let json = r#"{
    "EV_AGRTYPE_TEXT": "Teilleistungsbeurteilung",
    "EV_AUDTYPE_TEXT": "Abschluss eines Studienmoduls",
    "EV_CONTINUE_INDICATOR": "",
    "EV_DEREG_END": "2024-05-21",
    "EV_DEREG_ENDTIME": "13:00:00",
    "EV_DURATION": "120",
    "EV_DURUNIT": "MIN",
    "EV_EXAMBEGTIME": "13:00:00",
    "EV_EXAMDATE": "2024-05-22",
    "EV_EXAMENDTIME": "15:00:00",
    "EV_EXAMORG_TEXT": "KLSR",
    "EV_EXAMORG_LONGTEXT": "Klausur (K)",
    "EV_INSTRUCTOR": "Prof. Dr. Musterperson",
    "EV_LOCATION_SHORT": "Dresden",
    "EV_LOCATION_STEXT": "Berufsakademie Dresden",
    "EV_OBTYPE_TEXT": "Modulprüfung",
    "EV_REASON": "",
    "EV_REGIS_BEGIN": "2024-03-18",
    "EV_REGIS_BEGTIME": "07:00:00",
    "EV_REGIS_END": "2024-05-15",
    "EV_REGIS_ENDTIME": "07:00:00",
    "EV_ROOM_SHORT": "2.234",
    "EV_ROOM_STEXT": "Seminarraum 2.234",
    "EV_SHORT": "5CS-STDS-01",
    "EV_STEXT": "P1 Servers. Techn. u. vert. Systeme (SE)"
}"#;
    let exam_details: CdExamDetails = serde_json::from_str(json).unwrap();

    Ok(Json(exam_details))
}

pub async fn post_cancelexam(
    State(state): State<Arc<SimulatedState>>,
    Extension(_): Extension<CdAuthData>,
    Json(meta): Json<ExamRegistrationMetadata>,
) -> Result<String, ResponseError> {
    sleep_some().await;
    let mut lock = state.stds_signup_states.lock().unwrap();
    match meta.assessment.as_str() {
        "fake1" => lock.0 = false,
        "fake2" => lock.1 = false,
        _ => unreachable!(),
    };
    Ok("{}".to_string())
}

pub async fn get_examverfahren(
    State(state): State<Arc<SimulatedState>>,
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualVerfahrenOption>>, ResponseError> {
    let stds_signup_state = *state.stds_signup_states.lock().unwrap();

    sleep_some().await;
    let mut signup_verfahren = vec![];
    if stds_signup_state.0 {
        signup_verfahren.push(CampusDualVerfahrenOption {
            name: "P1 Servers. Techn. u. vert. Systeme (SE) (5CS-STDS-01)".to_string(),
            verfahren: "SEP".to_string(),
            pruefart: "Sommersemester".to_string(),
            status: "📝".to_string(),
            signup_information: "Prüfungsart: MPMA".to_string(),
            exam_date: Some("32.12.2024".to_string()),
            exam_time: Some("12:34".to_string()),
            exam_room: Some("Ort: 5SR 104".to_string()),
            warning_message: Some("Abmeldung ist nur noch bis morgen möglich.".to_string()),
            signoff_until: Some("31.12.2024".to_string()),
            internal_metadata: Some(ExamRegistrationMetadata {
                assessment: "fake1".to_string(),
                peryr: "".to_string(),
                perid: "".to_string(),
                offerno: "".to_string(),
            }),
        });
    };
    if stds_signup_state.1 {
        signup_verfahren.push(CampusDualVerfahrenOption {
            name: "P1 Servers. Techn. u. vert. Systeme (SE) (5CS-STDS-02)".to_string(),
            verfahren: "SEP".to_string(),
            pruefart: "Sommersemester".to_string(),
            status: "📝".to_string(),
            signup_information: "Prüfungsart: MPMA".to_string(),
            exam_date: Some("33.12.2024".to_string()),
            exam_time: Some("12:34".to_string()),
            exam_room: Some("Ort: 5SR 104".to_string()),
            warning_message: Some("Abmeldung ist nur noch bis morgen möglich.".to_string()),
            signoff_until: Some("31.12.2024".to_string()),
            internal_metadata: Some(ExamRegistrationMetadata {
                assessment: "fake2".to_string(),
                peryr: "".to_string(),
                perid: "".to_string(),
                offerno: "".to_string(),
            }),
        });
    };
    signup_verfahren.push(CampusDualVerfahrenOption {
        name: "Andere Prüfung".to_string(),
        verfahren: "Verfahren".to_string(),
        pruefart: "Prüfungsart".to_string(),
        status: "🚫".to_string(),
        signup_information: "Wichtige Info".to_string(),
        exam_date: Some("32.12.2024".to_string()),
        exam_time: Some("12:34".to_string()),
        exam_room: Some("SSR 123".to_string()),
        warning_message: Some("Abmeldung war nur noch bis gestern möglich".to_string()),
        signoff_until: Some("31.12.2024".to_string()),
        internal_metadata: None,
    });

    Ok(Json(signup_verfahren))
}

pub async fn get_ects(Extension(_): Extension<CdAuthData>) -> Result<String, ResponseError> {
    sleep_some().await;
    Ok("155".to_string())
}

pub async fn get_fachsem(Extension(_): Extension<CdAuthData>) -> Result<String, ResponseError> {
    sleep_some().await;
    Ok("6".to_string())
}

pub async fn get_examstats(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<CdExamStats>, ResponseError> {
    sleep_some().await;
    // CAMPUSDUAL PIECHART:
    // daten/partitionen: ['erfolgreich', 0], ['nicht bestanden', 0], ['gebucht', 0]
    // farben: ["#0070a3", "#4297d7", "#fcbe04"]

    let resp = CdExamStats {
        total: 32,
        successful: 29,
        unsuccessful: 3,
        unassessed: 0,
        booked: 0,
        finished: 0,
        ronmodus: 0,
    };

    Ok(Json(resp))
}

pub async fn get_stundenplan(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Vec<StundenplanItem>>, ResponseError> {
    sleep_some().await;
    sleep_some().await;
    sleep_some().await;
    sleep_some().await;

    let mut stundenplan: Vec<StundenplanItem> = vec![];

    let today = Utc::now().date_naive();
    let days_range = -3..=3;

    for offset in days_range {
        let date = today + Duration::days(offset);
        let eight = date.and_hms_opt(8, 0, 0).unwrap().and_utc().timestamp();
        let ninethirty = date.and_hms_opt(9, 30, 0).unwrap().and_utc().timestamp();
        let ten = date.and_hms_opt(10, 0, 0).unwrap().and_utc().timestamp();
        let eleventhirty = date.and_hms_opt(11, 30, 0).unwrap().and_utc().timestamp();
        let twelve = date.and_hms_opt(12, 0, 0).unwrap().and_utc().timestamp();
        let thirteen = date.and_hms_opt(13, 0, 0).unwrap().and_utc().timestamp();

        stundenplan.push(StundenplanItem {
            all_day: false,
            color: "egal".to_string(),
            font_color: None,
            description: "Beschreibung".to_string(),
            editable: false,
            end: ninethirty,
            instructor: "Dozent".to_string(),
            remarks: "Ein Hinweis".to_string(),
            room: "103 Seminarraum".to_string(),
            sinstructor: "DZNT".to_string(),
            sroom: "103 SR".to_string(),
            start: eight,
            title: "n-ZSPM1".to_string(),
        });
        stundenplan.push(StundenplanItem {
            all_day: false,
            color: "egal".to_string(),
            font_color: None,
            description: "Beschreibung".to_string(),
            editable: false,
            end: eleventhirty,
            instructor: "Dozent".to_string(),
            remarks: "".to_string(),
            room: "103 Seminarraum".to_string(),
            sinstructor: "DZNT".to_string(),
            sroom: "103 SR".to_string(),
            start: ten,
            title: "n-ZSPM2".to_string(),
        });

        stundenplan.push(StundenplanItem {
            all_day: false,
            color: "egal".to_string(),
            font_color: None,
            description: "Beschreibung".to_string(),
            editable: false,
            end: thirteen,
            instructor: "Dozent".to_string(),
            remarks: "Online".to_string(),
            room: "103 Seminarraum".to_string(),
            sinstructor: "DZNT".to_string(),
            sroom: "103 SR".to_string(),
            start: twelve,
            title: "n-ZSPM2".to_string(),
        });
    }

    for item in &mut stundenplan {
        item.start *= 1000;
        item.end *= 1000;
        item.color = match item.color.as_str() {
            "darkred" => "#D41610".to_string(),
            _ => string_to_rgb(&format!("0{}0", item.title)),
        };
        item.font_color = Some(
            if hex_to_luminance(&item.color) < 128.0 {
                "#FFFFFF"
            } else {
                "#000000"
            }
            .to_string(),
        );
    }

    Ok(Json(stundenplan))
}

fn string_to_rgb(input: &str) -> String {
    // Create a hasher
    let mut hasher = FnvHasher::default();

    // Hash the input string
    input.hash(&mut hasher);
    let hash = hasher.finish();

    // Extract RGB components from the hash
    let r = (hash & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = ((hash >> 16) & 0xFF) as u8;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub async fn get_reminders(
    State(state): State<Arc<SimulatedState>>,
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<CampusReminders>, ResponseError> {
    sleep_some().await;
    let lock = *state.stds_signup_states.lock().unwrap();
    let json = if lock.0 && lock.1 {
        r#"{
    "ELECTIVES": 0,
    "EXAMS": 0,
    "LATEST": [
        {
            "ACAD_SESSION": "Sommerperiode",
            "ACAD_YEAR": "Akad. Jahr 2023/2024",
            "AGRDATE": "20240717",
            "AGRTYPE": "Teilleistungsbeurteilung",
            "AWOBJECT": "P Integrierte Informationssysteme (C)",
            "AWOBJECT_SHORT": "5CS-ERPS-00",
            "AWOTYPE": "Studienmodul",
            "AWSTATUS": "Erfolgreich abgeschlossen",
            "BOOKDATE": "20240718",
            "BOOKREASON": "",
            "CPGRADED": "  0.00000",
            "CPUNIT": "ECTS-Credits",
            "GRADESYMBOL": "1,3"
        }
    ],
    "SEMESTER": 7,
    "UPCOMING": [
        {
            "BEGUZ": "090000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "110000",
            "EVDAT": "20240903",
            "INSTRUCTOR": "diverse",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-MEDIT-00",
            "SM_STEXT": "P Medizinisches Informationsmanagem. (K)",
            "SROOM": "5SR 104"
        },
        {
            "BEGUZ": "090000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "100000",
            "EVDAT": "20240905",
            "INSTRUCTOR": "Prof. Dr. Mustermann, Prof.in Dr.in Musterfrau",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-V3DA-01",
            "SM_STEXT": "P1 Videotech., 3D-Modell. u. Animat. (K)",
            "SROOM": ""
        },
        {
            "BEGUZ": "080000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "000000",
            "EVDAT": "20240910",
            "INSTRUCTOR": "diverse",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-V3DA-02",
            "SM_STEXT": "P2 Videotech., 3D-Modell. u. Animat. (P)",
            "SROOM": "5SR 104"
        },
        {
            "BEGUZ": "080000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "000000",
            "EVDAT": "20240912",
            "INSTRUCTOR": "diverse",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-STDS-02",
            "SM_STEXT": "P2 Servers. Techn. u. vert. Systeme (P)",
            "SROOM": "5SR 104"
        },
        {
            "BEGUZ": "080000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "000000",
            "EVDAT": "20240912",
            "INSTRUCTOR": "diverse",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-STDS-01",
            "SM_STEXT": "P1 Servers. Techn. u. vert. Systeme (SE)",
            "SROOM": "5SR 104"
        },
        {
            "BEGUZ": "130000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "000000",
            "EVDAT": "20240926",
            "INSTRUCTOR": "Prof.in Dr.in Musterfrau",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-BACS-00",
            "SM_STEXT": "P Bachelorarbeit SG Informatik",
            "SROOM": "5SR 104"
        }
    ]
}"#
    } else {
        r#"{
    "ELECTIVES": 0,
    "EXAMS": 1,
    "LATEST": [
        {
            "ACAD_SESSION": "Sommerperiode",
            "ACAD_YEAR": "Akad. Jahr 2023/2024",
            "AGRDATE": "20240717",
            "AGRTYPE": "Teilleistungsbeurteilung",
            "AWOBJECT": "P Integrierte Informationssysteme (C)",
            "AWOBJECT_SHORT": "5CS-ERPS-00",
            "AWOTYPE": "Studienmodul",
            "AWSTATUS": "Erfolgreich abgeschlossen",
            "BOOKDATE": "20240718",
            "BOOKREASON": "",
            "CPGRADED": "  0.00000",
            "CPUNIT": "ECTS-Credits",
            "GRADESYMBOL": "1,3"
        }
    ],
    "SEMESTER": 7,
    "UPCOMING": [
        {
            "BEGUZ": "090000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "110000",
            "EVDAT": "20240903",
            "INSTRUCTOR": "diverse",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-MEDIT-00",
            "SM_STEXT": "P Medizinisches Informationsmanagem. (K)",
            "SROOM": "5SR 104"
        },
        {
            "BEGUZ": "090000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "100000",
            "EVDAT": "20240905",
            "INSTRUCTOR": "Prof. Dr. Mustermann, Prof.in Dr.in Musterfrau",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-V3DA-01",
            "SM_STEXT": "P1 Videotech., 3D-Modell. u. Animat. (K)",
            "SROOM": ""
        },
        {
            "BEGUZ": "080000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "000000",
            "EVDAT": "20240910",
            "INSTRUCTOR": "diverse",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-V3DA-02",
            "SM_STEXT": "P2 Videotech., 3D-Modell. u. Animat. (P)",
            "SROOM": "5SR 104"
        },
        {
            "BEGUZ": "080000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "000000",
            "EVDAT": "20240912",
            "INSTRUCTOR": "diverse",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-STDS-01",
            "SM_STEXT": "P1 Servers. Techn. u. vert. Systeme (SE)",
            "SROOM": "5SR 104"
        },
        {
            "BEGUZ": "130000",
            "COMMENT": "Prüfung (SEP)",
            "ENDUZ": "000000",
            "EVDAT": "20240926",
            "INSTRUCTOR": "Prof. Dr. Mustermann",
            "LOCATION": "",
            "OBJID": "00000000",
            "ROOM": "104 Seminarraum",
            "SINSTRUCTOR": "",
            "SM_SHORT": "5CS-BACS-00",
            "SM_STEXT": "P Bachelorarbeit SG Informatik",
            "SROOM": "5SR 104"
        }
    ]
}"#
    };

    let resp: CampusReminders = serde_json::from_str(json).unwrap();

    Ok(Json(resp))
}

pub async fn get_timeline(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<ExportTimelineEvents>, ResponseError> {
    sleep_some().await;

    let json = r##"{
    "fachsemester": [
        {
            "name": "1. FS",
            "description": "1. Fachsemester vom 01.10.2021 bis 27.03.2022",
            "color": "#fcbe04",
            "start": "2021-10-01T00:00:00+02:00",
            "end": "2022-03-27T00:00:00+01:00"
        },
        {
            "name": "2. FS",
            "description": "2. Fachsemester vom 28.03.2022 bis 25.09.2022",
            "color": "#fcbe04",
            "start": "2022-03-28T00:00:00+02:00",
            "end": "2022-09-25T00:00:00+02:00"
        },
        {
            "name": "3. FS",
            "description": "3. Fachsemester vom 26.09.2022 bis 26.03.2023",
            "color": "#fcbe04",
            "start": "2022-09-26T00:00:00+02:00",
            "end": "2023-03-26T00:00:00+01:00"
        },
        {
            "name": "4. FS",
            "description": "4. Fachsemester vom 27.03.2023 bis 24.09.2023",
            "color": "#fcbe04",
            "start": "2023-03-27T00:00:00+02:00",
            "end": "2023-09-24T00:00:00+02:00"
        },
        {
            "name": "5. FS",
            "description": "5. Fachsemester vom 25.09.2023 bis 24.03.2024",
            "color": "#fcbe04",
            "start": "2023-09-25T00:00:00+02:00",
            "end": "2024-03-24T00:00:00+01:00"
        },
        {
            "name": "6. FS",
            "description": "6. Fachsemester vom 25.03.2024 bis 30.09.2024",
            "color": "#fcbe04",
            "start": "2024-03-25T00:00:00+01:00",
            "end": "2024-09-30T00:00:00+02:00"
        }
    ],
    "theoriesemester": [
        {
            "name": "Theorie",
            "description": "Theoriephase 1. Fachsemester vom 27.12.2021 bis 27.03.2022",
            "color": "#0070a3",
            "start": "2021-12-27T00:00:00+01:00",
            "end": "2022-03-27T00:00:00+01:00"
        },
        {
            "name": "Theorie",
            "description": "Theoriephase 2. Fachsemester vom 27.06.2022 bis 25.09.2022",
            "color": "#0070a3",
            "start": "2022-06-27T00:00:00+02:00",
            "end": "2022-09-25T00:00:00+02:00"
        },
        {
            "name": "Theorie",
            "description": "Theoriephase 3. Fachsemester vom 26.12.2022 bis 26.03.2023",
            "color": "#0070a3",
            "start": "2022-12-26T00:00:00+01:00",
            "end": "2023-03-26T00:00:00+01:00"
        },
        {
            "name": "Theorie",
            "description": "Theoriephase 4. Fachsemester vom 26.06.2023 bis 24.09.2023",
            "color": "#0070a3",
            "start": "2023-06-26T00:00:00+02:00",
            "end": "2023-09-24T00:00:00+02:00"
        },
        {
            "name": "Theorie",
            "description": "Theoriephase 5. Fachsemester vom 01.01.2024 bis 24.03.2024",
            "color": "#0070a3",
            "start": "2024-01-01T00:00:00+01:00",
            "end": "2024-03-24T00:00:00+01:00"
        },
        {
            "name": "Theorie",
            "description": "Theoriephase 6. Fachsemester vom 17.06.2024 bis 15.09.2024",
            "color": "#0070a3",
            "start": "2024-06-17T00:00:00+02:00",
            "end": "2024-09-15T00:00:00+02:00"
        }
    ],
    "praxissemester": [
        {
            "name": "Praxis",
            "description": "Praxisphase 1. Fachsemester vom 01.10.2021 bis 26.12.2021",
            "color": "#119911",
            "start": "2021-10-01T00:00:00+02:00",
            "end": "2021-12-26T00:00:00+01:00"
        },
        {
            "name": "Praxis",
            "description": "Praxisphase 2. Fachsemester vom 28.03.2022 bis 26.06.2022",
            "color": "#119911",
            "start": "2022-03-28T00:00:00+02:00",
            "end": "2022-06-26T00:00:00+02:00"
        },
        {
            "name": "Praxis",
            "description": "Praxisphase 3. Fachsemester vom 26.09.2022 bis 25.12.2022",
            "color": "#119911",
            "start": "2022-09-26T00:00:00+02:00",
            "end": "2022-12-25T00:00:00+01:00"
        },
        {
            "name": "Praxis",
            "description": "Praxisphase 4. Fachsemester vom 27.03.2023 bis 25.06.2023",
            "color": "#119911",
            "start": "2023-03-27T00:00:00+02:00",
            "end": "2023-06-25T00:00:00+02:00"
        },
        {
            "name": "Praxis",
            "description": "Praxisphase 5. Fachsemester vom 25.09.2023 bis 31.12.2023",
            "color": "#119911",
            "start": "2023-09-25T00:00:00+02:00",
            "end": "2023-12-31T00:00:00+01:00"
        },
        {
            "name": "Praxis",
            "description": "Praxisphase 6. Fachsemester vom 25.03.2024 bis 16.06.2024",
            "color": "#119911",
            "start": "2024-03-25T00:00:00+01:00",
            "end": "2024-06-16T00:00:00+02:00"
        },
        {
            "name": "Praxis",
            "description": "Praxisphase 6. Fachsemester vom 16.09.2024 bis 30.09.2024",
            "color": "#119911",
            "start": "2024-09-16T00:00:00+02:00",
            "end": "2024-09-30T00:00:00+02:00"
        }
    ],
    "specials": [
        {
            "name": "Verteidigung Abschlussarbeit",
            "description": "Die Verteidigungen der Abschlussarbeiten (Bachelor oder Diplom) finden voraussichtlich im Zeitraum vom 23.09.2024 bis zum 30.09.2024 statt.",
            "color": "#880000",
            "start": "2024-09-23T00:00:00+02:00",
            "end": "2024-09-30T00:00:00+02:00"
        }
    ]
}"##;

    let export_events: ExportTimelineEvents = serde_json::from_str(json).unwrap();

    Ok(Json(export_events))
}

fn _events_by_color(color: &str, events: &[CampusTimelineEvent]) -> Vec<ExportTimelineEvent> {
    events
        .iter()
        .filter(|event| event.color == color)
        .map(|event| ExportTimelineEvent {
            name: event.title.clone(),
            description: event
                .description
                .replace("<br>", " ")
                .replace("<strong>", "")
                .replace("</strong>", ""),
            color: event.color.clone(),
            start: _campusdate_to_iso8601(&event.start),
            end: _campusdate_to_iso8601(&event.end),
        })
        .collect()
}

fn _campusdate_to_iso8601(input: &str) -> String {
    let format = "%a, %d %b %Y %H:%M:%S %z";

    let date_time = DateTime::parse_from_str(input, format).expect("Failed to parse date");
    date_time.to_rfc3339()
}
