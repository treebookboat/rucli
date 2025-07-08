//! エラー型の定義モジュール

use std::fmt;
use std::io;

/// rucliで使用するカスタムエラー型
///
/// 各種エラーを統一的に扱うための列挙型
#[derive(Debug)]
pub enum RucliError {
    /// コマンドのパース時に発生するエラー
    ParseError(String),

    /// ファイル操作時のI/Oエラー
    IoError(io::Error),

    /// 引数の数や形式が不正な場合のエラー
    InvalidArgument(String),

    /// 存在しないコマンドが入力された場合のエラー
    UnknownCommand(String),

    /// 無効な正規表現パターン
    InvalidRegex(String),
    // その他のエラー
    // Other(String),
}

impl fmt::Display for RucliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RucliError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            RucliError::IoError(err) => write!(f, "IO error: {err}"),
            RucliError::InvalidArgument(msg) => write!(f, "argument error: {msg}"),
            RucliError::UnknownCommand(msg) => write!(f, "unknown command error: {msg}"),
            RucliError::InvalidRegex(msg) => write!(f, "Invalid syntax error: {msg}"),
            // RucliError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl From<io::Error> for RucliError {
    fn from(error: io::Error) -> Self {
        RucliError::IoError(error)
    }
}

impl std::error::Error for RucliError {}

/// Result型のエイリアス
pub type Result<T> = std::result::Result<T, RucliError>;
