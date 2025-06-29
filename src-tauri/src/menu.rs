use crate::{
    add_font_size, is_english_locale, reduce_font_size, refresh_view, set_acrylic_theme,
    set_acrylic_theme_extra, set_mica_theme, set_tabbed_window_theme, show_all_files_from_results,
    show_archives_from_results, show_audio_files_from_results, show_documents_from_results,
    show_executables_from_results, show_folders_from_results, show_images_from_results,
    show_videos_from_results, sort_by_extension, sort_by_file_type, sort_by_modified_date,
    sort_by_name, sort_by_path, sort_by_size, start_search_with_case_sensitive,
    start_search_with_match_path, start_search_with_regex_search,
    start_search_with_whole_word_match, toggle_auto_start_when_turn_on,
    toggle_check_update_on_startup, toggle_filter_bar, toggle_preview_pane, toggle_status_bar,
};
use tauri::menu::{CheckMenuItem, CheckMenuItemBuilder, MenuBuilder, MenuEvent, SubmenuBuilder};
use tauri::{App, AppHandle, Manager, Wry};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

const REGEX_SYNTAX: &str = r#"
正则表达式语法: 
	a|b		 匹配 a 或 b
	gr(a|e)y		 匹配 gray 或 grey
	.		 匹配任一字符
	[abc]		 匹配任一字符: a 或 b 或 c
	[^abc]		 匹配任一字符, 但不包括 a, b, c
	[a-z]		 匹配从 a 到 z 之间的任一字符
	[a-zA-Z]		 匹配从 a 到 z, 及从 A 到 Z 之间的任一字符
	^		 匹配文件名的头部
	$		 匹配文件名的尾部
	( )		 匹配标记的子表达式
	\n		 匹配第 nth 个标记的子表达式, nth 代表 1 到 9
	\b		 匹配字词边界
	*		 匹配前一项内容 0 或多次
	?		 匹配前一项内容 0 或 1 次
	+		 匹配前一项内容 1 或多次
	*?		 匹配前一项内容 0 或多次 (懒人模式)
	+?		 匹配前一项内容 1 或多次 (懒人模式)
	{x}		 匹配前一项内容 x 次
	{x,}		 匹配前一项内容 x 或多次
	{x,y}		 匹配前一项内容次数介于 x 和 y 之间
	\		 特殊转义字符
"#;

pub fn create_menu_options(app: &mut App) {
    let file_menu = SubmenuBuilder::new(
        app,
        if is_english_locale() {
            "File"
        } else {
            "文件"
        },
    )
    .text(
        "hide",
        if is_english_locale() {
            "Hide Window"
        } else {
            "隐藏窗口,Ctrl+Alt+W"
        },
    )
    .text(
        "show",
        if is_english_locale() {
            "Show All Windows"
        } else {
            "显示窗口,Ctrl+Alt+W"
        },
    )
    .text(
        "quit",
        if is_english_locale() {
            "Quit"
        } else {
            "退出"
        },
    )
    .build()
    .unwrap();

    let help_menu = SubmenuBuilder::new(
        app,
        if is_english_locale() {
            "Help"
        } else {
            "帮助"
        },
    )
    .text(
        "about",
        if is_english_locale() {
            "About"
        } else {
            "关于"
        },
    )
    .text(
        "repo",
        if is_english_locale() {
            "Open Repository Homepage"
        } else {
            "打开仓库主页"
        },
    )
    .text(
        "regex",
        if is_english_locale() {
            "Regular Expressions Syntax"
        } else {
            "正则表达式语法"
        },
    )
    .build()
    .unwrap();

    let check_update = CheckMenuItemBuilder::new(if is_english_locale() {
        "Check for Updates on Startup"
    } else {
        "启动时检查更新"
    })
    .id("check_update")
    .checked(true)
    .build(app)
    .unwrap();

    let auto_start_when_turn_on = CheckMenuItemBuilder::new(if is_english_locale() {
        "Auto Start When Turn On"
    } else {
        "开机自启"
    })
    .id("auto_start_when_turn_on")
    .checked(false)
    .build(app)
    .unwrap();
    let settings_menu = SubmenuBuilder::new(
        app,
        if is_english_locale() {
            "Settings"
        } else {
            "设置"
        },
    )
    .item(&check_update)
    .item(&auto_start_when_turn_on)
    .build()
    .unwrap();

    let edit_menu = SubmenuBuilder::new(
        app,
        if is_english_locale() {
            "Edit"
        } else {
            "编辑"
        },
    )
    .text(
        "cut",
        if is_english_locale() {
            "Cut"
        } else {
            "剪切 Ctrl+X"
        },
    )
    .text(
        "copy",
        if is_english_locale() {
            "Copy"
        } else {
            "复制 Ctrl+C"
        },
    )
    .text(
        "paste",
        if is_english_locale() {
            "Paste"
        } else {
            "粘贴 Ctrl+V"
        },
    )
    .text(
        "select_all",
        if is_english_locale() {
            "Select All"
        } else {
            "全选 Ctrl+A"
        },
    )
    .build()
    .unwrap();
    let name = CheckMenuItemBuilder::new(if is_english_locale() {
        "Name"
    } else {
        "名称"
    })
    .id("name")
    .checked(true)
    .build(app)
    .unwrap();
    let path = CheckMenuItemBuilder::new(if is_english_locale() {
        "Path"
    } else {
        "路径"
    })
    .id("path")
    .checked(false)
    .build(app)
    .unwrap();
    let size = CheckMenuItemBuilder::new(if is_english_locale() {
        "Size"
    } else {
        "大小"
    })
    .id("size")
    .checked(false)
    .build(app)
    .unwrap();
    let extension = CheckMenuItemBuilder::new(if is_english_locale() {
        "Extension"
    } else {
        "扩展名"
    })
    .id("extension")
    .checked(false)
    .build(app)
    .unwrap();
    let file_type = CheckMenuItemBuilder::new(if is_english_locale() {
        "File Type"
    } else {
        "文件类型"
    })
    .id("file_type")
    .checked(false)
    .build(app)
    .unwrap();
    let modified_date = CheckMenuItemBuilder::new(if is_english_locale() {
        "Modified Date"
    } else {
        "修改日期"
    })
    .id("modified_date")
    .checked(false)
    .build(app)
    .unwrap();

    let sort_menu = SubmenuBuilder::new(
        app,
        if is_english_locale() {
            "Sort"
        } else {
            "排序"
        },
    )
    .items(&[&name, &path, &size, &extension, &file_type, &modified_date])
    .build()
    .unwrap();

    let filter_bar = CheckMenuItemBuilder::new(if is_english_locale() {
        "Filter Bar"
    } else {
        "筛选栏"
    })
    .id("filter")
    .checked(true)
    .build(app)
    .unwrap();
    let preview_bar = CheckMenuItemBuilder::new(if is_english_locale() {
        "Preview Pane"
    } else {
        "预览栏"
    })
    .id("preview")
    .checked(false)
    .build(app)
    .unwrap();
    let status_bar = CheckMenuItemBuilder::new(if is_english_locale() {
        "Status Bar"
    } else {
        "状态栏"
    })
    .id("status_bar")
    .checked(true)
    .build(app)
    .unwrap();

    let view_menu = SubmenuBuilder::new(
        app,
        if is_english_locale() {
            "View"
        } else {
            "视图"
        },
    )
    .item(&filter_bar)
    .item(&preview_bar)
    .item(&status_bar)
    .text(
        "add_font_size",
        if is_english_locale() {
            "Add Font Size"
        } else {
            "增加字体大小"
        },
    )
    .text(
        "reduce_font_size",
        if is_english_locale() {
            "Reduce Font Size"
        } else {
            "减少字体大小"
        },
    )
    .text(
        "refresh",
        if is_english_locale() {
            "Refresh"
        } else {
            "刷新"
        },
    )
    .separator()
    .item(&sort_menu)
    .build()
    .unwrap();

    let case_sensitive = CheckMenuItemBuilder::new(if is_english_locale() {
        "Case Sensitive"
    } else {
        "区分大小写"
    })
    .id("case_sensitive")
    .checked(false)
    .build(app)
    .unwrap();
    let whole_word_match = CheckMenuItemBuilder::new(if is_english_locale() {
        "Whole Word Match"
    } else {
        "全词匹配"
    })
    .id("whole_word_match")
    .checked(false)
    .build(app)
    .unwrap();
    let match_path = CheckMenuItemBuilder::new(if is_english_locale() {
        "Match Path"
    } else {
        "匹配路径"
    })
    .id("match_path")
    .checked(false)
    .build(app)
    .unwrap();
    let regex_search = CheckMenuItemBuilder::new(if is_english_locale() {
        "Regular Expressions Search"
    } else {
        "正则表达式匹配搜索"
    })
    .id("regex_search")
    .checked(false)
    .build(app)
    .unwrap();

    let all_files = CheckMenuItemBuilder::new(if is_english_locale() {
        "All Files"
    } else {
        "所有文件"
    })
    .id("all_files")
    .checked(true)
    .build(app)
    .unwrap();
    let folders = CheckMenuItemBuilder::new(if is_english_locale() {
        "Folders"
    } else {
        "文件夹"
    })
    .id("folders")
    .checked(false)
    .build(app)
    .unwrap();
    let audio = CheckMenuItemBuilder::new(if is_english_locale() {
        "Audio"
    } else {
        "音频"
    })
    .id("audio")
    .checked(false)
    .build(app)
    .unwrap();
    let documents = CheckMenuItemBuilder::new(if is_english_locale() {
        "Documents"
    } else {
        "文档"
    })
    .id("documents")
    .checked(false)
    .build(app)
    .unwrap();
    let executables = CheckMenuItemBuilder::new(if is_english_locale() {
        "Executables"
    } else {
        "可执行文件"
    })
    .id("executables")
    .checked(false)
    .build(app)
    .unwrap();
    let images = CheckMenuItemBuilder::new(if is_english_locale() {
        "Images"
    } else {
        "图像"
    })
    .id("images")
    .checked(false)
    .build(app)
    .unwrap();
    let videos = CheckMenuItemBuilder::new(if is_english_locale() {
        "Videos"
    } else {
        "视频"
    })
    .id("videos")
    .checked(false)
    .build(app)
    .unwrap();
    let archives = CheckMenuItemBuilder::new(if is_english_locale() {
        "Archives"
    } else {
        "压缩文件"
    })
    .id("archives")
    .checked(false)
    .build(app)
    .unwrap();

    let search_menu = SubmenuBuilder::new(
        app,
        if is_english_locale() {
            "Search"
        } else {
            "搜索"
        },
    )
    .items(&[&case_sensitive, &whole_word_match, &match_path])
    .separator()
    .item(&regex_search)
    .separator()
    .items(&[
        &all_files,
        &folders,
        &audio,
        &documents,
        &executables,
        &images,
        &videos,
        &archives,
    ])
    .build()
    .unwrap();

    let theme_menu = SubmenuBuilder::new(
        app,
        if is_english_locale() {
            "Theme"
        } else {
            "主题"
        },
    )
    .text(
        "mica",
        if is_english_locale() {
            "Mica Theme"
        } else {
            "米卡主题"
        },
    )
    .text(
        "acrylic",
        if is_english_locale() {
            "Acrylic Theme"
        } else {
            "亚克力主题"
        },
    )
    .text(
        "acrylic2",
        if is_english_locale() {
            "Acrylic2 Theme"
        } else {
            "亚克力2主题"
        },
    )
    .text(
        "tabbed",
        if is_english_locale() {
            "Tabbed Theme"
        } else {
            "选项卡主题"
        },
    )
    .build()
    .unwrap();

    let menu = MenuBuilder::new(app)
        .items(&[
            &file_menu,
            &edit_menu,
            &view_menu,
            &search_menu,
            &settings_menu,
            &help_menu,
            &theme_menu,
        ])
        .build()
        .unwrap();

    app.set_menu(menu).unwrap();

    let check_items = [
        check_update,
        auto_start_when_turn_on,
        name,
        path,
        size,
        extension,
        file_type,
        modified_date,
        filter_bar,
        preview_bar,
        status_bar,
        case_sensitive,
        whole_word_match,
        match_path,
        regex_search,
        folders,
        audio,
        documents,
        executables,
        images,
        videos,
        archives,
        all_files,
    ];
    app.on_menu_event(move |app_handle: &AppHandle, event| {
        println!("menu event: {:?}", event.id());
        handle_menu_event(app_handle, event, check_items.to_vec());
    });
}

fn handle_menu_event(
    app_handle: &AppHandle,
    event: MenuEvent,
    check_items: Vec<CheckMenuItem<Wry>>,
) {
    let sort_items = check_items[2..=7].to_vec(); // 包含右边界
    let show_result_items = check_items[14..].to_vec();
    let name = check_items[2].clone();
    let path = check_items[3].clone();

    match event.id().0.as_str() {
        "show" => {
            let window = app_handle.get_webview_window("main").unwrap();
            window.show().unwrap();
            window.show_menu().unwrap();
        }
        "hide" => {
            let window = app_handle.get_webview_window("main").unwrap();
            window.hide().unwrap();
        }
        "quit" => {
            app_handle.exit(0);
        }
        "about" => {
            let _ = app_handle
                .dialog()
                .message(
                    r#" 
                        Author  :  SuperWindcloud  <<ss1178933440@gmail.com>>
                        Version :  1.1.1
                        Website :  https://github.com/super1windcloud/everything
                        License :  MIT License
                        Everything is a file manager that allows you to organize your 
                        files into a hierarchy and perform a variety of tasks on them.
                    "#,
                )
                .kind(MessageDialogKind::default())
                .title(if is_english_locale() {
                    "About"
                } else {
                    "关于"
                })
                .blocking_show();
        }
        "regex" => {
            let _ = app_handle
                .dialog()
                .message(REGEX_SYNTAX)
                .title(if is_english_locale() {
                    "Regular Expressions Syntax"
                } else {
                    "正则表达式语法"
                })
                .blocking_show();
        }
        "repo" => {
            let url = "https://github.com/super1windcloud/everything";
            let _ = webbrowser::open(url);
        }
        "mica" => set_mica_theme(app_handle),
        "acrylic" => set_acrylic_theme(app_handle),
        "tabbed" => set_tabbed_window_theme(app_handle),
        "acrylic2" => set_acrylic_theme_extra(app_handle),
        "add_font_size" => add_font_size(),
        "reduce_font_size" => reduce_font_size(),
        "refresh" => refresh_view(),
        "name" => sort_by_name(name, sort_items.as_slice()),
        "path" => sort_by_path(path, sort_items.as_slice()),
        "size" => sort_by_size(sort_items.as_slice()),
        "extension" => sort_by_extension(sort_items.as_slice()),
        "file_type" => sort_by_file_type(sort_items.as_slice()),
        "modified_date" => sort_by_modified_date(sort_items.as_slice()),
        "filter" => toggle_filter_bar(check_items.as_slice()),
        "preview" => toggle_preview_pane(check_items.as_slice()),
        "status_bar" => toggle_status_bar(check_items.as_slice()),
        "case_sensitive" => start_search_with_case_sensitive(check_items.as_slice()),
        "whole_word_match" => start_search_with_whole_word_match(check_items.as_slice()),
        "match_path" => start_search_with_match_path(check_items.as_slice()),
        "regex_search" => start_search_with_regex_search(check_items.as_slice()),
        "all_files" => show_all_files_from_results(show_result_items.as_slice()),
        "folders" => show_folders_from_results(show_result_items.as_slice()),
        "audios" => show_audio_files_from_results(show_result_items.as_slice()),
        "documents" => show_documents_from_results(show_result_items.as_slice()),
        "executables" => show_executables_from_results(show_result_items.as_slice()),
        "images" => show_images_from_results(show_result_items.as_slice()),
        "videos" => show_videos_from_results(show_result_items.as_slice()),
        "archives" => show_archives_from_results(show_result_items.as_slice()),
        "check_update" => toggle_check_update_on_startup(check_items.as_slice()),
        "auto_start_when_turn_on" => {
            toggle_auto_start_when_turn_on(app_handle, check_items.as_slice())
        }

        _ => {}
    }
}

// 双击打开路径在文件管理器中
