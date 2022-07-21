#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::process::Command;

fn start_vpn(){
  let _output = if cfg!(target_os = "windows") {
    Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process");
  } else {
    Command::new("/usr/local/bin/vpnutil")
    .arg("start")
    .arg("lt.fuckrkn1.xyz")
    .output()
    .expect("failed to execute process");
  };
}

fn stop_vpn() {
  let _output = if cfg!(target_os = "windows") {
    Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process");
  } else {
    Command::new("/usr/local/bin/vpnutil")
    .arg("stop")
    .arg("lt.fuckrkn1.xyz")
    .output()
    .expect("failed to execute process");
  };
}

#[tauri::command]
fn call_rust(is_connect: bool) -> bool { 
  if is_connect {
    start_vpn()
  } else {
    stop_vpn()
  }
  is_connect
}
fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![call_rust])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
