#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::process::Command;

fn start_vpn() -> bool {
    if cfg!(target_os = "windows") {
        let str = r#"IKEv2 VPN lt.fuckrkn1.xyz"#;
        let output = Command::new("rasdial")
            .arg(str)
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            true
        } else {
            false
        }
    } else if cfg!(target_os = "macos") {
        let output = Command::new("/usr/local/bin/vpnutil")
            .arg("start")
            .arg("lt.fuckrkn1.xyz")
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn stop_vpn() -> bool {
    if cfg!(target_os = "windows") {
        let output = Command::new("rasdial")
            .arg("/disconnect")
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            false
        } else {
            true
        }
    } else if cfg!(target_os = "macos") {
        let output = Command::new("/usr/local/bin/vpnutil")
            .arg("stop")
            .arg("lt.fuckrkn1.xyz")
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            false
        } else {
            true
        }
    } else {
        false
    }
}

#[tauri::command]
fn call_rust(is_connect: bool) -> bool {
    let mut status = true;
    if is_connect {
        status = start_vpn()
    } else {
        status = stop_vpn()
    }
    status
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![call_rust])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
