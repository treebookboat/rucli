use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use std::vec;

/// セッション固有の環境変数ストレージ
static SESSION_VARS: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 環境変数を取得
pub fn get_var(name: &str) -> Option<String> {
    // SESSION_VARSをロックして取得
    let session_vars = SESSION_VARS.lock().unwrap();

    // セッション変数から検索
    if let Some(value) = session_vars.get(name) {
        return Some(value.clone());
    }

    // セッション変数になければシステム変数から検索
    std::env::var(name).ok()
}

/// 環境変数を設定
pub fn set_var(name: &str, value: &str) {
    // SESSION_VARSをロックして取得
    let mut session_vars = SESSION_VARS.lock().unwrap();
    session_vars.insert(name.to_string(), value.to_string());
}

// 環境変数をすべて表示
pub fn list_all_vars() -> Vec<(String, String)> {
    let mut result = Vec::new();

    // システム環境変数を取得
    for (key, value) in std::env::vars() {
        result.push((key, value));
    }

    // セッション変数を取得
    let session_vars = SESSION_VARS.lock().unwrap();

    for (key, value) in session_vars.iter() {
        // 既存のシステム変数があれば上書き、なければ追加
        if let Some(pos) = result.iter().position(|(k, _)| k == key) {
            result[pos] = (key.clone(), value.clone());
        } else {
            result.push((key.clone(), value.clone()));
        }
    }

    result
}
