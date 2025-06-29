use crate::is_english_locale;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
use tauri::{App, Manager};
use tauri_plugin_dialog::DialogExt;

pub fn create_tray(app: &mut App) {
    use tauri::{
        menu::{Menu, MenuItem},
        tray::TrayIconBuilder,
    };

    let quit = MenuItem::with_id(
        app,
        "quit",
        if is_english_locale() {
            "Quit"
        } else {
            "退出"
        },
        true,
        None::<&str>,
    )
    .unwrap();
    let options = MenuItem::with_id(
        app,
        "options",
        if is_english_locale() {
            "Options"
        } else {
            "选项"
        },
        true,
        None::<&str>,
    )
    .unwrap();
    let open_file_list = MenuItem::with_id(
        app,
        "open_file_list",
        if is_english_locale() {
            "Open File List"
        } else {
            "打开文件列表"
        },
        true,
        None::<&str>,
    )
    .unwrap();
    let show_search = MenuItem::with_id(
        app,
        "show_search",
        if is_english_locale() {
            "Show Search Window"
        } else {
            "显示搜索窗口"
        },
        true,
        None::<&str>,
    )
    .unwrap();
    let menu = Menu::with_items(app, &[&open_file_list, &show_search, &options, &quit]).unwrap();

    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("🦀 Everything!")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }

            "open_file_list" => {
                let _file_path = app.dialog().file().blocking_pick_file();
            }
            "show_search" => {
                let window = app.get_webview_window("main").unwrap();
                window.show().unwrap();
                window.show_menu().unwrap();
                window.set_focus().unwrap();
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                #[cfg(debug_assertions)]
                println!("left click pressed and released");
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                        window.hide_menu().unwrap();
                    } else {
                        window.show().unwrap();
                        window.show_menu().unwrap();
                        window.set_focus().unwrap();
                    }
                }
            }

            _ => {}
        })
        .show_menu_on_left_click(false)
        .build(app)
        .unwrap();
}
