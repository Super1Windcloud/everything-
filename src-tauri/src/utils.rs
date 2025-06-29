use chrono::{DateTime, FixedOffset, Utc};
use infer::MatcherType;
use std::time::{Duration, UNIX_EPOCH};
use sys_locale::get_locale;

pub fn get_system_locale() -> String {
    let locale = get_locale().unwrap_or_default();
    if locale.is_empty() {
        "en_US".to_string()
    } else if locale == "zh-CN" {
        "zh_CN".to_string()
    } else {
        "en_US".to_string()
    }
}

pub fn is_english_locale() -> bool {
    if get_system_locale() == "en_US" {
        true
    } else {
        false
    }
}

pub fn convert_human_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        let size = size / 1024;
        format!("{:.2} KB", size as f64)
    } else if size < 1024 * 1024 * 1024 {
        let size = size / 1024 / 1024;
        format!("{:.2} MB", size as f64)
    } else {
        let size = size / 1024 / 1024 / 1024;
        format!("{:.2} GB", size as f64)
    }
}

pub fn convert_human_date(windows_ticks: u64, file_path: &str) -> String {
    const EPOCH_DIFF: u64 = 116444736000000000;

    let unix_nanos = windows_ticks
        .checked_sub(EPOCH_DIFF)
        .and_then(|diff| diff.checked_mul(100))
        .unwrap_or_else(|| {
            eprintln!("path : {file_path}"); 
            eprintln!(
                "❌ 无法转换时间戳：windows_ticks={} 小于基准或溢出",
                windows_ticks
            );
            return 0;
        });

    if unix_nanos == 0 {
        return "Invalid timestamp".to_string();
    }

    let utc_datetime =
        DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_nanos(unix_nanos));
    let date = utc_datetime
        .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    date
}


pub fn judge_file_type(file_path: &str, extension: &str) -> String {
    if extension.is_empty() {
        "folder".into()
    } else {
        let kind = infer::get_from_path(file_path).unwrap_or_default();
        if kind.is_none() {
            "text".into()
        } else {
            let kind = kind.unwrap();
            match kind.matcher_type() {
                MatcherType::App => "exe".into(),
                MatcherType::Archive => "archive".into(),
                MatcherType::Audio => "audio".into(),
                MatcherType::Book => "document".into(),
                MatcherType::Doc => "document".into(),
                MatcherType::Font => "font".into(),
                MatcherType::Image => "image".into(),
                MatcherType::Text => "text".into(),
                MatcherType::Video => "video".into(),
                MatcherType::Custom => "shortcut".into(),
            }
        }
    }
}

#[test]
fn test_get_system_locale() {
    let locale = get_system_locale();
    assert!(!locale.is_empty());
}
