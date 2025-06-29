#![allow(clippy::unnecessary_cast)]

use colored::Colorize;
use std::path::Path;
use toml_edit::{DocumentMut, Item, Table, Value};
const DEFAULT_CONFIG: &str = r#"
[view]
filter = true
preview= false 
status_bar = true
font_size = 12
sort_by = "name"

[search]
case_sensitive = false
whole_word_match = false
match_path = false
regex_match = false
show_file_type =  "all" 

"#;

#[tauri::command]
pub fn init_config_file() -> String {
    let home = std::env::var("HOME")
        .or(std::env::var("USERPROFILE"))
        .unwrap();
    let config_file_path = format!("{}\\.config\\everything.toml", home);
    if !Path::new(&config_file_path).exists() {
        std::fs::File::create(&config_file_path).unwrap();
        let config = DEFAULT_CONFIG.parse::<DocumentMut>().unwrap();
        let config_str = config.to_string();
        std::fs::write(&config_file_path, config_str).unwrap();
    }
    config_file_path
}

#[tauri::command]
pub fn read_config_content() -> String {
    let config_path = init_config_file();
    let config = std::fs::read_to_string(&config_path).unwrap();
    config
}

pub fn add_config_value(parent: Option<&str>, key: &str, value: &str) {
    let config_file_path = init_config_file();
    let config = std::fs::read_to_string(&config_file_path).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let value = value
        .parse::<Value>()
        .or_else(|_| Ok(Value::from(value)) as Result<Value, String>)
        .unwrap();
    if parent.is_some() {
        let parent = parent.unwrap();
        let table = if config.contains_table(parent) {
            config.get_mut(parent).unwrap().as_table_mut().unwrap()
        } else {
            config
                .entry(parent)
                .or_insert(Item::Table(Table::new()))
                .as_table_mut()
                .unwrap()
        };

        table.insert(key, value.into());
    } else {
        if config.contains_key(key) {
            return;
        } else {
            config[key] = value.into();
        }
    }
    std::fs::write(&config_file_path, config.to_string()).unwrap();
}

pub fn update_config_value(parent: Option<&str>, key: &str, value: &str) {
    let config_file_path = init_config_file();
    let config = std::fs::read_to_string(&config_file_path).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    let value = value
        .parse::<Value>()
        .or_else(|_| Ok(Value::from(value)) as Result<Value, String>)
        .unwrap();
    if parent.is_some() {
        let parent = parent.unwrap();
        if config.contains_table(parent) {
            let table = config.get_mut(parent).unwrap().as_table_mut().unwrap();
            table.insert(key, value.into());
        } else {
            eprintln!("{}", "Parent table not found".red().bold().to_string());
        }
    } else {
        if config.contains_key(key) && !config.contains_table(key) {
            config[key] = value.into()
        }
    }

    std::fs::write(&config_file_path, config.to_string()).unwrap();
}

pub fn rm_config_value(parent: Option<&str>, key: &str) {
    let config_file_path = init_config_file();
    let config = std::fs::read_to_string(&config_file_path).unwrap();
    let mut config = config.parse::<DocumentMut>().unwrap();
    if parent.is_some() {
        let parent = parent.unwrap();
        let table = config.get_mut(parent).unwrap().as_table_mut().unwrap();
        if table.contains_key(key) {
            table.remove(key);
        }
        if table.len() == 0 {
            config.remove(parent);
        }
    } else {
        if let Some(_) = config.get_mut(key) {
            config.remove(key);
        }
    }
    std::fs::write(&config_file_path, config.to_string()).unwrap();
}

mod test_config {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_init_config_file() {
        init_config_file();
        add_config_value(Some("super"), "bar", "242");
        add_config_value(Some("super"), "foo", "242");
        add_config_value(Some("super"), "aaa", "bbb");
        // rm_config_value(Some("super"), "foo");
        // rm_config_value(Some("super"), "jiasoef");
        update_config_value(Some("super"), "bar", "false");
        update_config_value(Some("super"), "foo", "dkdkdk");
        update_config_value(Some("super"), "aaa", "123");
    }
}
