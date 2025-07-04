use std::fmt;
use std::io;

#[derive(Debug)]
pub enum RucliError {
    // パースエラー
    ParseError(String),
    
    // ファイル操作エラー
    IoError(io::Error),
    
    // 引数エラー
    InvalidArgument(String),
    
    // コマンドが見つからない
    UnknownCommand(String),
    
    // その他のエラー
    // Other(String),
}

impl fmt::Display for RucliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RucliError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            RucliError::IoError(err) => write!(f, "IO error: {}", err),
            RucliError::InvalidArgument(msg) => write!(f, "argument error: {}", msg),
            RucliError::UnknownCommand(msg) => write!(f, "unknown command error: {}", msg),
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

pub type Result<T> = std::result::Result<T, RucliError>;