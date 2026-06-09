use winreg::enums::*;
use winreg::RegKey;

#[tauri::command]
pub fn add_user_env_var(key: String, value: String) -> Result<bool, String> {
    log::info!(
        "[envirment] Writing environment variable to HKCU\\Environment using winreg crate: {}={}",
        key,
        value
    );

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let (env_key, _disp) = hkcu.create_subkey("Environment").map_err(|e| {
        let error_msg = format!("Failed to open HKCU\\Environment registry key: {}", e);
        log::error!("[envirment] {}", error_msg);
        error_msg
    })?;

    env_key.set_value(&key, &value).map_err(|e| {
        let error_msg = format!("Failed to set registry value: {}", e);
        log::error!("[envirment] {}", error_msg);
        error_msg
    })?;

    log::info!(
        "[envirment] Successfully wrote registry value {} to HKCU\\Environment.",
        key
    );

    let broadcast_script = r#"
        $signature = '[DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)] public static extern IntPtr SendMessageTimeout(IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam, uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);'
        $type = Add-Type -MemberDefinition $signature -Name "Win32SendMessage" -Namespace "Win32" -PassThru
        $result = [UIntPtr]::Zero
        [void]$type::SendMessageTimeout([IntPtr]0xffff, 0x001a, [UIntPtr]::Zero, "Environment", 2, 5000, [ref]$result)
    "#;

    if let Err(e) = std::process::Command::new("powershell.exe")
        .arg("-Command")
        .arg(broadcast_script)
        .output()
    {
        log::warn!("[envirment] Failed to broadcast WM_SETTINGCHANGE: {}", e);
    } else {
        log::info!("[envirment] Successfully broadcasted WM_SETTINGCHANGE message to all windows.");
    }

    Ok(true)
}
