// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_accent_color() -> Vec<u8> {
    let mut colorization: u32 = 0;
    let mut opaqueblend = windows::Win32::Foundation::BOOL(0);
    unsafe {
        let _ = windows::Win32::Graphics::Dwm::DwmGetColorizationColor(
            &mut colorization,
            &mut opaqueblend,
        );
    }; // this can return an error

    let argb = hex::decode(format!("{:X}", colorization)).unwrap();
    println!("{:?}", argb);
    argb
}

use tauri::Manager;
use tauri::Theme;
use window_vibrancy::apply_mica;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = apply_mica(&window, Some(Theme::Dark != Theme::Light));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_accent_color, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
