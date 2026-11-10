// ============================================
// Модуль: launcher
// Надёжный запуск UWP-приложений через cmd /c start
// ============================================

use std::process::Command;

// ВАЖНО: Теперь окончание !Game, как показал PowerShell
pub const ROBLOX_AUMID: &str = "ROBLOXCorporation.RobloxGDK_55nm5eh3cm0pr!Game";

pub fn launch_roblox() -> std::io::Result<u32> {
    // Используем cmd.exe /c start для запуска приложения по AUMID.
    let child = Command::new("cmd.exe")
        .args(&["/C", "start", &format!("shell:appsFolder\\{}", ROBLOX_AUMID)])
        .spawn()?;

    let pid = child.id();
    Ok(pid)
}