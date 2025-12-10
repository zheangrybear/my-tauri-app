mod lingti;

use lingti::*;
use tauri::{command};

#[command]
fn start_tun_with_config_file(config: String) -> i32 {
    start_tun2r_with_config_file(&config)
}

#[command]
fn start_tun(config: String) -> i32 {
    start_tun2r(&config)
}

#[command]
fn stop_tun() -> i32 {
    stop_tun2r()
}

#[command]
fn sdk_version() -> String {
    get_version()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_tun_with_config_file,
            start_tun,
            stop_tun,
            sdk_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri");
}
