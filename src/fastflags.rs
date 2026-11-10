// ============================================
// Модуль: fastflags
// Управление профилями оптимизации
// ============================================

use std::fs;
use std::path::PathBuf;

pub const FLAGS_PERFORMANCE: &str = r#"{
  "DFFlagTextureQualityOverrideEnabled": "True",
  "DFIntTextureQualityOverride": "1",
  "FFlagHandleAltEnterFullscreenManually": "False",
  "FFlagDebugSkyGray": "True",
  "DFFlagDisableDPIScale": "True",
  "DFIntDebugFRMQualityLevelOverride": "1",
  "FIntDebugForceMSAASamples": "1",
  "FIntFRMMinGrassDistance": "0",
  "FIntFRMMaxGrassDistance": "0",
  "DFIntCSGLevelOfDetailSwitchingDistance": "0",
  "FIntRenderShadowIntensity": "0"
}"#;

pub const FLAGS_BALANCED: &str = r#"{
  "DFIntTextureQualityOverride": "2",
  "DFIntDebugFRMQualityLevelOverride": "5",
  "FIntFRMMaxGrassDistance": "100",
  "FIntRenderShadowIntensity": "50"
}"#;

pub const FLAGS_QUALITY: &str = r#"{
  "DFIntTextureQualityOverride": "3"
}"#;

pub fn write_fastflags(roblox_appdata: &PathBuf, profile: &str) -> std::io::Result<PathBuf> {
    let settings_dir = roblox_appdata.join("ClientSettings");
    fs::create_dir_all(&settings_dir)?;
    
    let content = match profile {
        "fps" => FLAGS_PERFORMANCE,
        "balanced" => FLAGS_BALANCED,
        "quality" => FLAGS_QUALITY,
        _ => FLAGS_PERFORMANCE,
    };
    
    let file_path = settings_dir.join("ClientAppSettings.json");
    fs::write(&file_path, content)?;
    
    Ok(file_path)
}