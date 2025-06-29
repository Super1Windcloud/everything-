use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn register_global_shortcuts(app: &mut App) {
    let hide_or_show_shortcut =
        Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyW);

    app.handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_app, shortcut, event| {
                    if shortcut == &hide_or_show_shortcut {
                        match event.state() {
                            ShortcutState::Pressed => {
                                println!("Ctrl-Alt-W Pressed!");
                            }
                            ShortcutState::Released => {
                                let window = _app.get_webview_window("main").unwrap();
                                if window.is_visible().unwrap() {
                                    window.hide().unwrap();
                                } else {
                                    window.show().unwrap();
                                }
                            }
                        }
                    } else {
                    }
                })
                .build(),
        )
        .unwrap();

    app.global_shortcut()
        .register(hide_or_show_shortcut)
        .unwrap();
}
