use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use anyhow::{Result, anyhow};
use base64::prelude::*;
use http::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey};
use rand::Rng;
use std::{env, str};

use crate::constants::AES_KEY;

fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    let mut rng = rand::rng();
    rng.fill(&mut nonce);
    nonce
}

pub fn encrypt(plaintext: &str) -> Result<(String, String), StatusCode> {
    let key = AES_KEY.get().unwrap();
    let cipher = Aes256Gcm::new(key.into());

    let nonce = generate_nonce();
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        BASE64_STANDARD.encode(nonce),
        BASE64_STANDARD.encode(ciphertext),
    ))
}

pub fn decrypt(nonce: &str, ciphertext: &str) -> Result<String> {
    let key = Key::<Aes256Gcm>::from_slice(AES_KEY.get().unwrap());
    let cipher = Aes256Gcm::new(key);

    let nonce = BASE64_STANDARD.decode(nonce)?;
    let ciphertext = BASE64_STANDARD.decode(ciphertext)?;

    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| anyhow!("decrypt fail"))?;

    Ok(String::from_utf8(plaintext)?)
}

pub fn get_aes_from_env() -> [u8; 32] {
    let key = match env::var("AES_KEY") {
        Err(_) => {
            log::error!("Environment variable AES_KEY is missing");
            std::process::exit(1);
        }
        Ok(key) => {
            if key.len() < 32 {
                log::error!("AES_KEY must be at least 32 bytes long.");
                std::process::exit(1);
            } else {
                key
            }
        }
    };

    let key_bytes = key.as_bytes();
    let mut key_array = [0u8; 32];
    if key_bytes.len() > 32 {
        key_array.copy_from_slice(&key_bytes[..32]);
    } else {
        key_array.copy_from_slice(key_bytes);
    }
    key_array
}

pub fn get_jwt_keys_from_env() -> (EncodingKey, DecodingKey) {
    let secret = match env::var("JWT_SECRET") {
        Err(_) => {
            log::error!("Environment variable JWT_SECRET is missing");
            std::process::exit(1);
        }
        Ok(key) => key,
    };

    (
        EncodingKey::from_secret(secret.as_ref()),
        DecodingKey::from_secret(secret.as_ref()),
    )
}
