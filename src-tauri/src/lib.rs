pub mod menu;
pub mod tray;
pub use menu::*;
pub mod utils;
pub use utils::*;
pub mod shortcut;
pub use shortcut::*;
pub mod operations;
pub mod system;
pub use operations::*;
pub mod config;
pub use config::*;
pub use system::*;
use tauri::{App, AppHandle, Manager};
pub use tray::*;
use window_vibrancy::{apply_acrylic, apply_mica, apply_tabbed};

#[tauri::command]
fn show_window(window: tauri::Window) -> Result<(), String> {
    if window.is_visible().unwrap() {
        return Ok(());
    }
    window.center().unwrap();
    window.show_menu().unwrap();

    window
        .show()
        .map_err(|e| format!("Failed to show window: {}", e))?;
    window
        .set_focus()
        .map_err(|e| format!("Failed to set focus: {}", e))?;
    Ok(())
}


pub fn set_window_theme(app: &mut App) {
    let window = app.get_webview_window("main").unwrap();
    #[cfg(target_os = "windows")]
    // apply_acrylic(&window, Some((18, 18, 18, 125))).unwrap();
    apply_mica(&window, None).unwrap()
}

pub fn set_acrylic_theme(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    #[cfg(target_os = "windows")]
    apply_acrylic(&window, None).unwrap();
}
pub fn set_acrylic_theme_extra(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    #[cfg(target_os = "windows")]
    apply_acrylic(&window, Some((18, 18, 18, 125))).unwrap();
}

pub fn set_mica_theme(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    #[cfg(target_os = "windows")]
    apply_mica(&window, None).unwrap();
}

pub fn set_tabbed_window_theme(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    #[cfg(target_os = "windows")]
    apply_tabbed(&window, None).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            show_window,
            request_everything_ipc,
            init_config_file,
            open_path_with_explorer,
            copy_to_clipboard,
            copy_to_file,
            delete_to_file,
            open_properties
        ])
        .setup(|app| {
            create_tray(app);
            set_window_theme(app);
            create_menu_options(app);
            register_global_shortcuts(app);
            return Ok(());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
