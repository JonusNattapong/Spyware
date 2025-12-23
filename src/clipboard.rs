use clipboard::{ClipboardContext, ClipboardProvider};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_clipboard_monitor(clipboard_data: Arc<Mutex<String>>) {
    thread::spawn(move || {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        loop {
            if let Ok(content) = ctx.get_contents() {
                let mut clip = clipboard_data.lock().unwrap();
                if content != *clip {
                    *clip = content.clone();
                }
            }
            thread::sleep(Duration::from_secs(5));
        }
    });
}