use crate::{add_config_value, init_config_file, is_english_locale, update_config_value};
use tauri::menu::CheckMenuItem;
use tauri::{AppHandle, Wry};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use toml_edit::DocumentMut;

pub fn add_font_size() {
    let config_file = init_config_file();
    let config = std::fs::read_to_string(config_file.as_str()).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let view_table = config.as_table_mut().get_mut("view").unwrap();
    let font_size = view_table.get("font_size").unwrap().as_integer().unwrap();

    let font_size = font_size + 2;

    view_table["font_size"] = font_size.into();
    std::fs::write(config_file, config.to_string()).unwrap();
}

pub fn reduce_font_size() {
    let config_file = init_config_file();
    let config = std::fs::read_to_string(config_file.as_str()).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let view_table = config.as_table_mut().get_mut("view").unwrap();
    let font_size = view_table.get("font_size").unwrap().as_integer().unwrap();
    let font_size = font_size - 2;

    view_table["font_size"] = font_size.into();
    std::fs::write(config_file, config.to_string()).unwrap();
}

pub fn refresh_view() {}

pub fn sort_by_name(name: CheckMenuItem<Wry>, sort_items: &[CheckMenuItem<Wry>]) {
    sort_items
        .iter()
        .for_each(|item| item.set_checked(false).unwrap());
    name.set_checked(true).unwrap();
    let config_file = init_config_file();
    let config = std::fs::read_to_string(config_file.as_str()).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let view_table = config.as_table_mut().get_mut("view").unwrap();

    view_table["sort_by"] = "name".into();
    std::fs::write(config_file, config.to_string()).unwrap();
}

pub fn sort_by_path(path: CheckMenuItem<Wry>, sort_items: &[CheckMenuItem<Wry>]) {
    sort_items
        .iter()
        .for_each(|item| item.set_checked(false).unwrap());
    path.set_checked(true).unwrap();
    let config_file = init_config_file();
    let config = std::fs::read_to_string(config_file.as_str()).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let view_table = config.as_table_mut().get_mut("view").unwrap();

    view_table["sort_by"] = "path".into();
    std::fs::write(config_file, config.to_string()).unwrap();
}

pub fn sort_by_size(sort_items: &[CheckMenuItem<Wry>]) {
    sort_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "size" {
            item.set_checked(true).unwrap();
        } else {
            item.set_checked(false).unwrap();
        }
    });
    let config_file = init_config_file();
    let config = std::fs::read_to_string(config_file.as_str()).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let view_table = config.as_table_mut().get_mut("view").unwrap();

    view_table["sort_by"] = "size".into();
    std::fs::write(config_file, config.to_string()).unwrap();
}
pub fn sort_by_file_type(sort_items: &[CheckMenuItem<Wry>]) {
    sort_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "file_type" {
            item.set_checked(true).unwrap();
        } else {
            item.set_checked(false).unwrap();
        }
    });
    let config_file = init_config_file();
    let config = std::fs::read_to_string(config_file.as_str()).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let view_table = config.as_table_mut().get_mut("view").unwrap();

    view_table["sort_by"] = "file_type".into();
    std::fs::write(config_file, config.to_string()).unwrap();
}

pub fn sort_by_modified_date(sort_items: &[CheckMenuItem<Wry>]) {
    sort_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "modified_date" {
            item.set_checked(true).unwrap();
        } else {
            item.set_checked(false).unwrap();
        }
    });
    let config_file = init_config_file();
    let config = std::fs::read_to_string(config_file.as_str()).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let view_table = config.as_table_mut().get_mut("view").unwrap();

    view_table["sort_by"] = "modified_date".into();
    std::fs::write(config_file, config.to_string()).unwrap();
}

pub fn sort_by_extension(sort_items: &[CheckMenuItem<Wry>]) {
    sort_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "extension" {
            item.set_checked(true).unwrap();
        } else {
            item.set_checked(false).unwrap();
        }
    });
    let config_file = init_config_file();
    let config = std::fs::read_to_string(config_file.as_str()).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let view_table = config.as_table_mut().get_mut("view").unwrap();

    view_table["sort_by"] = "extension".into();
    std::fs::write(config_file, config.to_string()).unwrap();
}

pub fn toggle_filter_bar(check_items: &[CheckMenuItem<Wry>]) {
    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "filter" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                update_config_value(Some("view"), "filter", "false");
            } else {
                item.set_checked(true).unwrap();
                update_config_value(Some("view"), "filter", "true");
            }
        }
    });
}

pub fn toggle_preview_pane(check_items: &[CheckMenuItem<Wry>]) {
    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "preview" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                update_config_value(Some("view"), "preview", "false");
            } else {
                item.set_checked(true).unwrap();
                update_config_value(Some("view"), "preview", "true");
            }
        }
    });
}

pub fn toggle_status_bar(check_items: &[CheckMenuItem<Wry>]) {
    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "status_bar" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                update_config_value(Some("view"), "status_bar", "false");
            } else {
                item.set_checked(true).unwrap();
                update_config_value(Some("view"), "status_bar", "true");
            }
        }
    });
}

pub fn start_search_with_case_sensitive(check_items: &[CheckMenuItem<Wry>]) {
    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "case_sensitive" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                update_config_value(Some("search"), "case_sensitive", "false");
            } else {
                item.set_checked(true).unwrap();
                update_config_value(Some("search"), "case_sensitive", "true");
            }
        }
    });
}

pub fn start_search_with_whole_word_match(check_items: &[CheckMenuItem<Wry>]) {
    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "whole_word_match" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                update_config_value(Some("search"), "whole_word_match", "false");
            } else {
                item.set_checked(true).unwrap();
                update_config_value(Some("search"), "whole_word_match", "true");
            }
        }
    });
}
pub fn start_search_with_match_path(check_items: &[CheckMenuItem<Wry>]) {
    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "match_path" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                update_config_value(Some("search"), "match_path", "false");
            } else {
                item.set_checked(true).unwrap();
                update_config_value(Some("search"), "match_path", "true");
            }
        }
    });
}

pub fn start_search_with_regex_search(check_items: &[CheckMenuItem<Wry>]) {
    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "regex_search" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                update_config_value(Some("search"), "regex_search", "false");
            } else {
                item.set_checked(true).unwrap();
                update_config_value(Some("search"), "regex_search", "true");
            }
        }
    });
}

pub fn show_all_files_from_results(show_result_items: &[CheckMenuItem<Wry>]) {
    show_result_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "all_files" {
            item.set_checked(true).unwrap();
            update_config_value(Some("search"), "show_file_type", "all_files");
        } else {
            item.set_checked(false).unwrap();
        }
    });
}

pub fn show_folders_from_results(show_result_items: &[CheckMenuItem<Wry>]) {
    show_result_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "folders" {
            item.set_checked(true).unwrap();
            update_config_value(Some("search"), "show_file_type", "folders");
        } else {
            item.set_checked(false).unwrap();
        }
    });
}

pub fn show_audio_files_from_results(show_result_items: &[CheckMenuItem<Wry>]) {
    show_result_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "audios" {
            item.set_checked(true).unwrap();
            update_config_value(Some("search"), "show_file_type", "audios");
        } else {
            item.set_checked(false).unwrap();
        }
    });
}

pub fn show_images_from_results(show_result_items: &[CheckMenuItem<Wry>]) {
    show_result_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "images" {
            item.set_checked(true).unwrap();
            update_config_value(Some("search"), "show_file_type", "images");
        } else {
            item.set_checked(false).unwrap();
        }
    });
}

pub fn show_videos_from_results(show_result_items: &[CheckMenuItem<Wry>]) {
    show_result_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "videos" {
            item.set_checked(true).unwrap();
            update_config_value(Some("search"), "show_file_type", "videos");
        } else {
            item.set_checked(false).unwrap();
        }
    });
}

pub fn show_documents_from_results(show_result_items: &[CheckMenuItem<Wry>]) {
    show_result_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "documents" {
            item.set_checked(true).unwrap();
            update_config_value(Some("search"), "show_file_type", "documents");
        } else {
            item.set_checked(false).unwrap();
        }
    });
}

pub fn show_executables_from_results(show_result_items: &[CheckMenuItem<Wry>]) {
    show_result_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "executables" {
            item.set_checked(true).unwrap();
            update_config_value(Some("search"), "show_file_type", "executables");
        } else {
            item.set_checked(false).unwrap();
        }
    })
}

pub fn show_archives_from_results(show_result_items: &[CheckMenuItem<Wry>]) {
    show_result_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "archives" {
            item.set_checked(true).unwrap();
            update_config_value(Some("search"), "show_file_type", "archives");
        } else {
            item.set_checked(false).unwrap();
        }
    })
}

pub fn toggle_check_update_on_startup(check_items: &[CheckMenuItem<Wry>]) {
    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "check_update" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                add_config_value(Some("settings"), "check_update", "false")
            } else {
                item.set_checked(true).unwrap();
                add_config_value(Some("settings"), "check_update", "true")
            }
        }
    });
}

pub fn toggle_auto_start_when_turn_on(app_handle: &AppHandle, check_items: &[CheckMenuItem<Wry>]) {
    use tauri_plugin_autostart::MacosLauncher;
    use tauri_plugin_autostart::ManagerExt;

    app_handle
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .unwrap();

    check_items.iter().for_each(|item| {
        let name = item.id().0.to_string();
        if name == "auto_start_when_turn_on" {
            if !item.is_checked().unwrap() {
                item.set_checked(false).unwrap();
                let autostart_manager = app_handle.autolaunch();
                let _ = autostart_manager.disable();

                let _ = app_handle
                    .dialog()
                    .message(if is_english_locale() {
                        "The program will not start automatically when you turn on your computer."
                    } else {
                        "开启自启已关闭"
                    })
                    .kind(MessageDialogKind::default())
                    .title(if is_english_locale() {
                        "Tips"
                    } else {
                        "提示"
                    })
                    .blocking_show();
            } else {
                item.set_checked(true).unwrap();
                let autostart_manager = app_handle.autolaunch();
                let _ = autostart_manager.enable();
                let _ = app_handle
                    .dialog()
                    .message(if is_english_locale() {
                        "The program will start automatically when you turn on your computer."
                    } else {
                        "程序将在你开机时自动启动。"
                    })
                    .kind(MessageDialogKind::default())
                    .title(if is_english_locale() {
                        "Tips"
                    } else {
                        "提示"
                    })
                    .blocking_show();
            }
        }
    });
}

mod ops_test {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_add_font_size() {
        add_font_size();
        reduce_font_size()
    }
}
