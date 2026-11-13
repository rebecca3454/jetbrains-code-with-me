//! Windows registry read helpers for Jetbrains Code With Me.

pub const APP_KEY: &str = "Software\\Jetbrains\\Jetbrains Code With Me";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Jetbrains Code With Me";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Jetbrains Code With Me") },
        RegValue { name: "Version",     value: String::from("v3.5.9") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Jetbrains Code With Me") },
        RegValue { name: "Publisher",       value: String::from("Jetbrains") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Jetbrains Code With Me\\uninstall.exe\"") },
    ]
}
