// src/alias.rs（新規ファイル）
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static ALIASES: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// エイリアスを取得
pub fn get_alias(name: &str) -> Option<String> {
    ALIASES.lock().unwrap().get(name).cloned()
}

/// エイリアスを設定
pub fn set_alias(name: &str, command: &str) {
    ALIASES
        .lock()
        .unwrap()
        .insert(name.to_string(), command.to_string());
}

/// 全エイリアスを取得
pub fn list_aliases() -> Vec<(String, String)> {
    ALIASES
        .lock()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}
