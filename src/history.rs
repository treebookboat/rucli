use crate::error::Result;
use log::debug;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
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
    pub fn _clear(&mut self) {
        self.commands.clear();
    }

    // 履歴を丸ごと置き換える
    pub fn set_commands(&mut self, commands: VecDeque<String>) {
        self.commands = commands;
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
pub fn _clear_history() {
    HISTORY.lock().unwrap()._clear();
}

// 現在の履歴を指定ファイル、もしくはデフォルトファイルに保存
pub fn save_history_to_file(file_path: Option<&str>) -> Result<()> {
    // ファイルパスの決定
    let file_path = if let Some(path) = file_path {
        PathBuf::from(path)
    } else {
        get_default_history_file()
    };

    // 親ディレクトリの存在確認と作成
    if let Some(parent_dir) = file_path.parent() {
        ensure_history_dir_exists(parent_dir)?;
    }

    //  現在の履歴データを取得
    let history_list = get_history_list();

    // ファイルに書き込み
    let mut file = std::fs::File::create(&file_path)?;
    for (_, cmd) in history_list {
        writeln!(file, "{cmd}")?;
    }

    // ファイルの明示的なフラッシュで即時変更反映
    file.flush()?;

    // 成功ログの出力
    debug!("History saved to: {}", file_path.display());

    Ok(())
}

/// ファイルから履歴を読み込む
pub fn load_history_from_file(file_path: Option<&str>) -> Result<()> {
    // ファイルパスの決定
    let file_path = if let Some(path) = file_path {
        PathBuf::from(path)
    } else {
        get_default_history_file()
    };

    // ファイル存在確認
    if !file_path.exists() {
        debug!("No {} file", file_path.display());
        return Ok(());
    }

    // ファイルの読み込み準備
    let file = std::fs::File::open(&file_path)?;
    let reader = BufReader::new(file);
    let mut file_history = Vec::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let trimmed = line.trim();

                // 空行はスキップ
                if trimmed.is_empty() {
                    continue;
                }
                file_history.push(trimmed.to_string());
            }
            Err(e) => {
                debug!("Failed to read line from history file: {e}");
                continue;
            }
        }
    }

    set_history_from_vec(file_history);

    // 成功ログの出力
    debug!("History loaded from: {}", file_path.display());

    Ok(())
}

// 履歴にコマンドを保存
fn set_history_from_vec(commands: Vec<String>) {
    let mut history = HISTORY.lock().unwrap();
    history.set_commands(VecDeque::from(commands));
}

// 環境変数またはカレントディレクトリ/.rucli_historyを返す
pub fn get_default_history_file() -> PathBuf {
    // 環境変数RUCLI_HISTFILEをチェック
    if let Ok(hist_path) = std::env::var("RUCLI_HISTFILE") {
        return PathBuf::from(hist_path);
    }

    // デフォルトはカレントディレクトリの.rucli_history
    PathBuf::from(".rucli_history")
}

/// 履歴を検索する
///
/// # Arguments
/// * `query` - 検索文字列
///
/// # Returns
/// * マッチした履歴のリスト（番号とコマンド）
pub fn search_history(query: &str) -> Vec<(usize, String)> {
    let mut history = get_history_list();

    // 最後の要素（現在実行中のコマンド）を除外
    if !history.is_empty() {
        history.pop();
    }

    // 文字を小文字で統一
    let query = query.to_lowercase();

    // 履歴で文字列を含んでいるものだけ残す
    history
        .into_iter()
        .filter(|(_, cmd)| cmd.to_lowercase().contains(&query))
        .collect()
}

// 必要に応じて親ディレクトリを作成
fn ensure_history_dir_exists(dir_path: &Path) -> Result<()> {
    // ディレクトリの存在確認
    if dir_path.exists() {
        return Ok(());
    }

    // ディレクトリの作成
    std::fs::create_dir_all(dir_path)?;

    // 作成成功ログの出力
    debug!("create history directory : {}", dir_path.display());

    Ok(())
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
        _clear_history();

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
        _clear_history();

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

        _clear_history();
        let list = get_history_list();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_history_order() {
        let _guard = TEST_MUTEX.lock().unwrap();
        _clear_history();

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
        _clear_history();

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
