use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

pub fn find_roblox_player() -> Option<PathBuf> {
    let xbox_content = PathBuf::from(r"D:\XboxGames\Roblox\Content");
    if xbox_content.exists() {
        if let Some(exe) = find_exe_recursive(&xbox_content) {
            return Some(exe);
        }
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey("Software\\Roblox\\RobloxPlayer") {
        if let Ok(path) = key.get_value::<String, _>("RobloxPlayerPath") {
            let exe_path = PathBuf::from(path);
            if exe_path.exists() {
                return Some(exe_path);
            }
        }
    }
    None
}

fn find_exe_recursive(dir: &PathBuf) -> Option<PathBuf> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_exe_recursive(&path) {
                    return Some(found);
                }
            } else if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "exe" {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                        if filename.contains("roblox") || filename.contains("windows10universal") {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn get_roblox_appdata() -> Option<PathBuf> {
    dirs::data_local_dir().map(|p| p.join("Roblox"))
}
