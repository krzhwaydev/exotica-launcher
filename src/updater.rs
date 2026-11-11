use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn check_update(current: &str) -> Option<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("ExoticaLauncher")
        .build().ok()?;
        
    // ВАЖНО: Убедись, что тут твой правильный ник и название репозитория!
    let url = "https://api.github.com/repos/krzhwaydev/exotica-launcher/releases/latest";
    
    let rel: Release = client.get(url).send().ok()?.json().ok()?;
    let latest = rel.tag_name.trim_start_matches('v');
    
    if latest != current {
        for a in rel.assets {
            if a.name.ends_with(".exe") {
                return Some(a.browser_download_url);
            }
        }
    }
    None
}

pub fn download_and_replace(url: &str) -> Result<(), String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("ExoticaLauncher")
        .timeout(std::time::Duration::from_secs(120)) // 2 минуты на скачивание
        .build().map_err(|e| e.to_string())?;
        
    let bytes = client.get(url).send().map_err(|e| e.to_string())?
        .bytes().map_err(|e| e.to_string())?;
        
    let tmp = std::env::temp_dir().join("exotica_new.exe");
    fs::write(&tmp, &bytes).map_err(|e| e.to_string())?;
    
    // Заменяем текущий файл и перезапускаем
    self_replace::self_replace(&tmp).map_err(|e| e.to_string())?;
    Ok(())
}