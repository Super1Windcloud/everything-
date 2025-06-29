use crate::{convert_human_date, convert_human_size, judge_file_type};
use copypasta::{ClipboardContext, ClipboardProvider};
use everything_sdk::{global, EverythingError, RequestFlags};
use serde::{Deserialize, Serialize};
use std::path::Path;
use windows::core::BOOL;
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use windows::Win32::System::Ole::CF_HDROP;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;
use windows::{core::PCWSTR, Win32::Foundation::*, Win32::UI::Shell::*};

#[repr(C)]
struct DROPFILES {
    p_files: u32,
    pt: POINT,
    f_nc: BOOL,
    f_wide: BOOL,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct EverythingResult {
    file_name: String,
    file_path: String,
    file_size: String,
    file_extension: String,
    file_type: String,
    file_modified_time: String,
    file_created_time: String,
    total_result_count: usize,
}

impl EverythingResult {
    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }
    pub fn get_file_size(&self) -> &str {
        &self.file_size
    }
    pub fn get_file_extension(&self) -> &str {
        &self.file_extension
    }
    pub fn get_file_type(&self) -> &str {
        &self.file_type
    }
    pub fn get_file_modified_time(&self) -> &str {
        &self.file_modified_time
    }
    pub fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.into();
    }

    pub fn new(
        file_path: &str,
        file_size: u64,
        file_modified_time: u64,
        file_extension: &str,
        total_result_count: usize,
        file_created_date: u64,
    ) -> EverythingResult {
        let file_size = convert_human_size(file_size);
        let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();

        let file_modified_time = convert_human_date(file_modified_time, file_path);
        let file_created_date = convert_human_date(file_created_date, file_path);

        let file_type = judge_file_type(file_path, file_extension);
        Self {
            file_name: file_name.to_string(),
            file_path: file_path.into(),
            file_size: file_size.into(),
            file_extension: file_extension.into(),
            file_type: file_type.into(),
            file_modified_time: file_modified_time.into(),
            file_created_time: file_created_date.into(),
            total_result_count,
        }
    }
}

#[tauri::command]
pub fn request_everything_ipc(query: &str) -> Vec<EverythingResult> {
    let mut everything = global().try_lock().unwrap();
    match everything.is_db_loaded() {
        Ok(false) => panic!("The Everything database has not been fully loaded now."),
        Err(EverythingError::Ipc) => panic!("Everything is required to run in the background."),
        _ => {
            let mut searcher = everything.searcher();
            searcher.set_search(query);
            searcher
                .set_request_flags(
                    RequestFlags::EVERYTHING_REQUEST_FILE_NAME
                        | RequestFlags::EVERYTHING_REQUEST_PATH
                        | RequestFlags::EVERYTHING_REQUEST_SIZE
                        | RequestFlags::EVERYTHING_REQUEST_EXTENSION
                        | RequestFlags::EVERYTHING_REQUEST_DATE_MODIFIED
                        | RequestFlags::EVERYTHING_REQUEST_DATE_CREATED,
                )
                .set_max(u32::MAX);

            let results = searcher.query();

            let total_num_results = results.total();
            println!("Total number of results: {}", total_num_results);

            let everything_result = results
                .into_iter()
                .map(|item| {
                    let file_path = item.filepath().unwrap();
                    let file_size = item.size().unwrap();
                    let file_extension = item.extension().unwrap();
                    let file_modified_date = item.date_modified().unwrap();
                    let file_created_date = item.date_created().unwrap();

                    EverythingResult::new(
                        file_path.as_path().to_str().unwrap(),
                        file_size,
                        file_modified_date,
                        file_extension.to_str().unwrap(),
                        total_num_results as usize,
                        file_created_date,
                    )
                })
                .collect::<Vec<_>>();

            everything_result
        }
    }
}

pub fn open_default_editor(file_path: &str) {
    let editor = std::env::var("EDITOR").unwrap_or_default();
    let editor = if !editor.is_empty() {
        editor
    } else {
        "notepad.exe".to_string()
    };
    let _child = std::process::Command::new(editor)
        .arg(file_path)
        .spawn()
        .unwrap();
}

#[tauri::command]
pub fn open_path_with_explorer(file_path: &str) {
    let is_dir = Path::new(file_path).is_dir();
    let explorer = std::env::var("EXPLORER").unwrap_or_default();
    let explorer = if !explorer.is_empty() {
        explorer
    } else {
        "explorer.exe".to_string()
    };

    let _ = if is_dir {
        std::process::Command::new(explorer)
            .arg(file_path)
            .spawn()
            .unwrap();
    } else {
        open_default_editor(file_path);
    };
}

#[tauri::command]
pub fn copy_to_clipboard(file_path: &str) {
    let mut ctx = ClipboardContext::new().unwrap();

    ctx.set_contents(file_path.to_owned()).unwrap();
}

#[tauri::command]
pub fn copy_to_file(file_path: &str) {
    unsafe {
        let mut path_u16: Vec<u16> = file_path.encode_utf16().collect();
        path_u16.push(0); // 单 \0 结束
        path_u16.push(0); // 双 \0 表示结束整个列表

        // 分配内存（DROPFILES + 路径字符串）
        let dropfiles_size = size_of::<DROPFILES>() + path_u16.len() * 2;
        let hglobal = GlobalAlloc(GMEM_MOVEABLE, dropfiles_size).unwrap();
        let ptr = GlobalLock(hglobal) as *mut u8;

        // 写入 DROPFILES 结构
        let drop = DROPFILES {
            p_files: size_of::<DROPFILES>() as u32,
            pt: POINT { x: 0, y: 0 },
            f_nc: BOOL(0),
            f_wide: BOOL(1), // UTF-16
        };

        std::ptr::copy_nonoverlapping(&drop as *const _ as *const u8, ptr, size_of::<DROPFILES>());

        let dst = ptr.add(size_of::<DROPFILES>()) as *mut u16;
        std::ptr::copy_nonoverlapping(path_u16.as_ptr(), dst, path_u16.len());

        let _= GlobalUnlock(hglobal);

        // 设置剪贴板数据
        OpenClipboard(Some(HWND(std::ptr::null_mut()))).unwrap();
        EmptyClipboard().unwrap();
        SetClipboardData(CF_HDROP.0 as u32, Some(HANDLE(hglobal.0))).unwrap();
        CloseClipboard().unwrap();
    }
}

#[tauri::command]
pub fn delete_to_file(file_path: &str) {
    if Path::new(file_path).is_dir() {
        std::fs::remove_dir_all(file_path).unwrap();
    } else {
        std::fs::remove_file(file_path).unwrap();
    }
}

#[tauri::command]
pub fn open_properties(file_path: &str) {
    let wide_path: Vec<u16> = file_path.encode_utf16().chain(Some(0)).collect();
    let verb: Vec<u16> = "properties".encode_utf16().chain(Some(0)).collect();

    let mut sei = SHELLEXECUTEINFOW {
        cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_INVOKEIDLIST,
        lpVerb: PCWSTR(verb.as_ptr()),
        lpFile: PCWSTR(wide_path.as_ptr()),
        nShow: SW_SHOW.0,
        ..Default::default()
    };

    unsafe {
        if ShellExecuteExW(&mut sei as *mut _).is_ok() {
            println!("✅ 属性窗口已打开");
        } else {
            eprintln!("❌ 打开失败，请确认路径和文件类型");
        }
    }
}

mod test_system {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_editor_open() {
        let path = r"C:\Users\superuse\.config";
        open_default_editor(path);
    }

    #[test]
    fn test_everything_ipc() {
        let result = request_everything_ipc("啊色粉");
        println!("{:?}", result);
    }

    #[test]
    fn test_copy_to_clipboard() {
        let path = r"C:\Users\superuse\.config\everything.toml";
        copy_to_clipboard(path);
    }

    #[test]
    fn open_path() {
        let path = r"C:\Users\superuse\.config\everything.toml";
        let dir = r"C:\Users\superuse\.config";
        open_path_with_explorer(dir);
    }

    #[test]
    fn test_open_properties() {
        let path = r"C:\Users\superuse\.config\everything.toml";
        let dir = r"C:\Users\superuse\.config";
        open_properties(dir);
    }

    #[test]
    fn test_copy_to_file() {
        let path = r"C:\Users\superuse\Downloads\Telegram Desktop";
        copy_to_file(path);
    }
}
