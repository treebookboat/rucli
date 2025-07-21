use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::Mutex;

// グローバルな履歴インスタンス
static HISTORY: Lazy<Mutex<History>> = Lazy::new(|| Mutex::new(History::new(1000)));

// コマンド履歴を保存する構造体
struct History {
    commands: VecDeque<String>, // 履歴を保存
    max_size: usize,            // 最大保存数
}

impl History {
    /// コンストラクタ
    fn new(max_size: usize) -> Self {
        History {
            commands: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    /// 履歴に追加
    pub fn add(&mut self, command: String) {
        // 空文字列であれば早期return
        if command.trim().is_empty() {
            return;
        }

        // 最後の命令と同じであれば早期return
        if matches!(self.commands.back(), Some(last) if last == &command) {
            return;
        }

        // 保持できる最大個数を超えていれば一番古い履歴を削除
        if self.commands.len() >= self.max_size {
            self.commands.pop_front();
        }

        // 最新の履歴にコマンド追加
        self.commands.push_back(command);
    }

    // 履歴リストを取得
    pub fn list(&self) -> Vec<(usize, String)> {
        self.commands
            .iter()
            .enumerate()
            .map(|(i, cmd)| (i + 1, cmd.clone()))
            .collect()
    }

    // 履歴リストの削除
    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

// 公開API

/// 履歴にコマンドを追加
pub fn add_history(command: String) {
    HISTORY.lock().unwrap().add(command);
}

pub fn get_history_list() -> Vec<(usize, String)> {
    HISTORY.lock().unwrap().list()
}

/// 履歴をクリア
pub fn clear_history() {
    HISTORY.lock().unwrap().clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // テスト用のロック（テストが同時実行されないようにする）
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_add_history() {
        let _guard = TEST_MUTEX.lock().unwrap();
        clear_history();

        add_history("test_add_1".to_string());
        add_history("test_add_2".to_string());

        let list = get_history_list();
        // 他のテストの影響を受けないように、追加した分だけ確認
        assert!(list.iter().any(|(_, cmd)| cmd == "test_add_1"));
        assert!(list.iter().any(|(_, cmd)| cmd == "test_add_2"));
    }

    #[test]
    fn test_no_duplicate() {
        let _guard = TEST_MUTEX.lock().unwrap();
        clear_history();

        add_history("test_dup".to_string());
        let count_before = get_history_list().len();

        add_history("test_dup".to_string());
        let count_after = get_history_list().len();

        assert_eq!(count_before, count_after);
    }

    #[test]
    fn test_empty_command() {
        let _guard = TEST_MUTEX.lock().unwrap();

        let count_before = get_history_list().len();

        add_history("".to_string());
        add_history("  ".to_string());

        let count_after = get_history_list().len();
        assert_eq!(count_before, count_after);
    }

    #[test]
    fn test_clear_history() {
        let _guard = TEST_MUTEX.lock().unwrap();

        add_history("test_clear_1".to_string());
        add_history("test_clear_2".to_string());

        clear_history();
        let list = get_history_list();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_history_order() {
        let _guard = TEST_MUTEX.lock().unwrap();
        clear_history();

        add_history("first".to_string());
        add_history("second".to_string());
        add_history("third".to_string());

        let list = get_history_list();
        // 最後の3つを確認
        let last_three: Vec<String> = list
            .iter()
            .rev()
            .take(3)
            .map(|(_, cmd)| cmd.clone())
            .collect();
        assert_eq!(last_three[2], "first");
        assert_eq!(last_three[1], "second");
        assert_eq!(last_three[0], "third");
    }

    #[test]
    fn test_max_size() {
        let _guard = TEST_MUTEX.lock().unwrap();
        clear_history();

        // 1001個追加
        for i in 0..1001 {
            add_history(format!("maxtest_{}", i));
        }

        let list = get_history_list();
        assert_eq!(list.len(), 1000);
        // 最初のものは削除されているはず
        assert!(!list.iter().any(|(_, cmd)| cmd == "maxtest_0"));
        assert!(list.iter().any(|(_, cmd)| cmd == "maxtest_1000"));
    }
}
