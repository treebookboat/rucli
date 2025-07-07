//! コマンドの定義と実行を管理するモジュール

use crate::error::Result;
use crate::handlers::*;
use log::debug;

/// 実行可能なコマンドを表す列挙型
#[derive(Debug)]
pub enum Command {
    /// ヘルプを表示
    Help,
    /// メッセージを出力
    Echo {
        message: String,
    },
    /// メッセージを繰り返し出力
    Repeat {
        count: i32,
        message: String,
    },
    /// ファイルの内容を表示
    Cat {
        filename: String,
    },
    /// ファイルに内容を書き込む
    Write {
        filename: String,
        content: String,
    },
    /// ディレクトリの内容を一覧表示
    Ls,
    /// ディレクトリを変更
    Cd {
        path: String,
    },
    /// 現在の作業ディレクトリを表示
    Pwd,
    /// ディレクトリを作成
    Mkdir {
        path: String,
        parents: bool,
    },
    /// ファイル/ディレクトリを削除
    Rm {
        path: String,
        recursive: bool,
        force: bool,
    },
    // ファイルをコピー
    Cp {
        source: String,
        destination: String,
    },
    /// プログラムを終了
    Exit,
}

/// コマンドのメタ情報を保持する構造体
pub struct CommandInfo {
    /// コマンド名（例: "echo", "cat"）
    pub name: &'static str,
    /// コマンドの説明文
    pub description: &'static str,
    /// コマンドの使い方
    pub usage: &'static str,
    /// コマンドの最小引数個数
    pub min_args: usize,
    /// コマンドの最大引数個数(無制限であればNone)
    pub max_args: Option<usize>,
}

/// 利用可能なコマンド一覧
pub const COMMANDS: &[CommandInfo] = &[
    CommandInfo {
        name: "help",
        description: "Show this help message",
        usage: "help",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "echo",
        description: "Display message",
        usage: "echo <message...>",
        min_args: 1,
        max_args: None,
    },
    CommandInfo {
        name: "cat",
        description: "Display file contents",
        usage: "cat <filename>",
        min_args: 1,
        max_args: Some(1),
    },
    CommandInfo {
        name: "write",
        description: "Write content to file",
        usage: "write <filename> <content...>",
        min_args: 2,
        max_args: None,
    },
    CommandInfo {
        name: "ls",
        description: "List directory contents",
        usage: "ls",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "repeat",
        description: "Repeat message count times",
        usage: "repeat <count> <message...>",
        min_args: 2,
        max_args: None,
    },
    CommandInfo {
        name: "exit",
        description: "Exit the program",
        usage: "exit",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "cd",
        description: "Change directory",
        usage: "cd <directory>",
        min_args: 0,
        max_args: Some(1),
    },
    CommandInfo {
        name: "quit",
        description: "Exit the program",
        usage: "quit",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "pwd",
        description: "output the current working directory",
        usage: "pwd",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "rm",
        description: "Remove files",
        usage: "rm <file>",
        min_args: 1,
        max_args: Some(2),
    },
    CommandInfo {
        name: "cp",
        description: "Copy files",
        usage: "cp <source> <destination>",
        min_args: 2,
        max_args: Some(2),
    },
    CommandInfo {
        name: "mkdir",
        description: "Make directories",
        usage: "mkdir <directory>",
        min_args: 1,
        max_args: Some(2),
    },
];

/// コマンドを実行する
///
/// # Errors
///
/// - ファイル操作系コマンドでI/Oエラーが発生した場合
/// - ファイルが存在しない、権限がない等
pub fn execute_command(command: Command) -> Result<()> {
    // コマンド実行開始を記録
    debug!("Executing command: {command:?}");

    match command {
        Command::Help => {
            handle_help();
            Ok(())
        }
        Command::Cat { filename } => handle_cat(&filename),
        Command::Echo { message } => {
            println!("{message}");
            Ok(())
        }
        Command::Write { filename, content } => handle_write(&filename, &content),
        Command::Repeat { count, message } => {
            handle_repeat(count, &message);
            Ok(())
        }
        Command::Ls => handle_ls(),
        Command::Cd { path } => handle_cd(&path),
        Command::Pwd => handle_pwd(),
        Command::Mkdir { path, parents } => handle_mkdir(&path, parents),
        Command::Rm {
            path,
            recursive,
            force,
        } => handle_rm(&path, recursive, force),
        Command::Cp {
            source,
            destination,
        } => handle_cp(&source, &destination),
        Command::Exit => {
            handle_exit();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //被りがないかチェック
    fn test_command_info_no_duplicates() {
        let mut names: Vec<&str> = COMMANDS.iter().map(|c| c.name).collect();
        names.sort();

        for i in 1..names.len() {
            assert_ne!(names[i], names[i - 1], "Duplicate {}", names[i]);
        }
    }

    #[test]
    // min/maxの論理エラーを検出
    fn test_command_info_valid_args() {
        for cmd in COMMANDS {
            if let Some(max) = cmd.max_args {
                assert!(cmd.min_args <= max)
            }
        }
    }
}
