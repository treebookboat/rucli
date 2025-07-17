use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use crate::commands::Command;

static FUNCTIONS: Lazy<Mutex<HashMap<String, Command>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// シェル関数を定義（保存）する
///
/// # Arguments
/// * `name` - 関数名
/// * `body` - 関数の本体となるコマンド
///
pub fn define_function(name: &str, body: Command) {
    // FUNCTIONSのロックを取得
    let mut functions = FUNCTIONS.lock().unwrap();

    // FUNCTIONSに登録
    functions.insert(name.to_string(), body);
}

/// 定義済みの関数を取得する
///
/// # Arguments
/// * `name` - 取得したい関数名
///
/// # Returns
/// * `Some(Command)` - 関数が存在する場合、本体のクローン
/// * `None` - 関数が存在しない場合
///
pub fn get_function(name: &str) -> Option<Command> {
    // FUNCTIONSのロックを取得
    let functions = FUNCTIONS.lock().unwrap();

    functions.get(name).cloned()
}

/// 指定された名前の関数が存在するかチェック
///
/// # Arguments
/// * `name` - チェックしたい関数名
///
/// # Returns
/// * `true` - 関数が存在する
/// * `false` - 関数が存在しない
///
pub fn is_function(name: &str) -> bool {
    // FUNCTIONSのロックを取得
    let functions = FUNCTIONS.lock().unwrap();

    // キーを所持しているか
    functions.contains_key(name)
}

/// 定義済みの全関数名を取得
///
/// # Returns
/// * 関数名のベクタ（アルファベット順でなくてもOK）
///
pub fn list_functions() -> Vec<String> {
    // FUNCTIONSのロックを取得
    let functions = FUNCTIONS.lock().unwrap();

    // イテレータを取得して集める
    functions.keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::Command;

    #[test]
    fn test_define_and_get_function() {
        // Given: echoコマンドを本体とする関数
        let body = Command::Echo {
            message: "Hello from function!".to_string(),
        };

        // When: 関数を定義
        define_function("test_func", body.clone());

        // Then: 取得できる
        let retrieved = get_function("test_func");
        assert!(retrieved.is_some());
        assert!(matches!(retrieved.unwrap(), Command::Echo { .. }));
    }

    #[test]
    fn test_is_function() {
        // Given: 関数を定義
        let body = Command::Echo {
            message: "test".to_string(),
        };
        define_function("exists", body);

        // Then: 存在チェック
        assert!(is_function("exists"));
        assert!(!is_function("not_exists"));
    }

    #[test]
    fn test_function_overwrite() {
        // Given: 最初の定義
        let body1 = Command::Echo {
            message: "First".to_string(),
        };
        define_function("overwrite_test", body1);

        // When: 同じ名前で再定義
        let body2 = Command::Echo {
            message: "Second".to_string(),
        };
        define_function("overwrite_test", body2);

        // Then: 新しい定義で上書きされる
        let retrieved = get_function("overwrite_test").unwrap();
        match retrieved {
            Command::Echo { message } => assert_eq!(message, "Second"),
            _ => panic!("Expected Echo command"),
        }
    }

    #[test]
    fn test_list_functions() {
        // Setup: いくつか関数を定義
        let echo_cmd = Command::Echo {
            message: "test".to_string(),
        };
        define_function("func1", echo_cmd.clone());
        define_function("func2", echo_cmd.clone());
        define_function("func3", echo_cmd);

        // When: 一覧取得
        let functions = list_functions();

        // Then: 定義した関数が含まれる
        assert!(functions.contains(&"func1".to_string()));
        assert!(functions.contains(&"func2".to_string()));
        assert!(functions.contains(&"func3".to_string()));
    }

    #[test]
    fn test_get_nonexistent_function() {
        // When: 存在しない関数を取得
        let result = get_function("nonexistent");

        // Then: None が返る
        assert!(result.is_none());
    }

    #[test]
    fn test_complex_function_body() {
        // Given: パイプラインを含む複雑なコマンド
        let body = Command::Pipeline {
            commands: vec!["echo hello".to_string(), "grep h".to_string()],
        };

        // When: 関数として定義
        define_function("complex_func", body);

        // Then: 正しく取得できる
        let retrieved = get_function("complex_func").unwrap();
        assert!(matches!(retrieved, Command::Pipeline { .. }));
    }
}
