//! コマンドの定義と実行を管理するモジュール

use crate::environment::expand_variables;
use crate::error::Result;
use crate::handlers::*;
use crate::parser::parse_command;
use crate::pipeline::{PipelineCommand, PipelineExecutor};
use crate::redirect::execute_redirect;
use log::debug;

/// コマンドの実行結果を表す列挙型
pub enum CommandResult {
    /// 通常のコマンド実行結果（出力文字列）
    Continue(String),
    /// プログラムの終了要求
    Exit,
}

#[derive(Debug, Clone)]
pub enum HistoryAction {
    List,           // 全履歴表示
    Search(String), // 検索
    Execute(usize), // 番号で実行
}

/// 実行可能なコマンドを表す列挙型
#[derive(Debug, Clone)]
pub enum Command {
    /// ヘルプを表示
    Help,
    /// メッセージを出力
    Echo { message: String },
    /// メッセージを繰り返し出力
    Repeat { count: i32, message: String },
    /// ファイルの内容を表示
    Cat { filename: String },
    /// ファイルに内容を書き込む
    Write { filename: String, content: String },
    /// ディレクトリの内容を一覧表示
    Ls,
    /// ディレクトリを変更
    Cd { path: String },
    /// 現在の作業ディレクトリを表示
    Pwd,
    /// ディレクトリを作成
    Mkdir { path: String, parents: bool },
    /// ファイル/ディレクトリを削除
    Rm {
        path: String,
        recursive: bool,
        force: bool,
    },
    /// ファイル/ディレクトリをコピー
    Cp {
        source: String,
        destination: String,
        recursive: bool,
    },
    /// ファイル/ディレクトリの移動
    Mv { source: String, destination: String },
    /// ファイルの検索
    Find {
        path: Option<String>, // 検索開始ディレクトリ(何もなければホームポジション)
        name: String,         // 検索するファイル名
    },
    /// ファイル内のテキスト検索
    Grep { pattern: String, files: Vec<String> },
    /// アライアス設定
    Alias {
        name: Option<String>,
        command: Option<String>,
    },
    /// パイプラインコマンド
    Pipeline { commands: Vec<String> },
    /// バージョン表示
    Version,
    /// リダイレクト付きコマンド
    Redirect {
        command: Box<Command>, // 実行するコマンド
        redirect_type: String, // ">", ">>", "<"
        target: String,        // ファイル名
    },
    /// バックグラウンド実行
    Background { command: Box<Command> },
    /// スリープ
    Sleep { seconds: u64 },
    /// ジョブ一覧表示
    Jobs,
    /// フォアグラウンド処理切り替え
    Fg { job_id: Option<u32> },
    /// 環境変数コマンド
    Environment { action: EnvironmentAction },
    /// if条件分岐
    If {
        condition: Box<Command>,         // 条件コマンド
        then_part: Box<Command>,         // 成功時の処理
        else_part: Option<Box<Command>>, // 失敗時の処理（オプション）
    },
    /// While繰り返し
    While {
        condition: Box<Command>,
        body: Box<Command>,
    },
    /// For繰り返し
    For {
        variable: String,
        items: Vec<String>,
        body: Box<Command>,
    },
    /// 関数定義
    Function { name: String, body: Box<Command> },
    /// 関数呼び出し
    FunctionCall { name: String, args: Vec<String> },
    /// 複数のコマンドを順次実行
    Compound { commands: Vec<Command> },
    /// 履歴を表示
    History { action: HistoryAction },
    /// プログラムを終了
    Exit,
}

/// 環境変数のアクション
#[derive(Debug, Clone)]
pub enum EnvironmentAction {
    List,                // env
    Show(String),        // env VAR
    Set(String, String), // env VAR=value
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
        min_args: 0,
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
        max_args: Some(3),
    },
    CommandInfo {
        name: "mv",
        description: "Move/rename files or directories",
        usage: "mv <source> <destination>",
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
    CommandInfo {
        name: "grep",
        description: "Search for pattern in files",
        usage: "grep <pattern> <file...>",
        min_args: 1,
        max_args: None, // 複数ファイル対応
    },
    CommandInfo {
        name: "alias",
        description: "Set or show command aliases",
        usage: "alias [name=command]",
        min_args: 0,
        max_args: Some(1),
    },
    CommandInfo {
        name: "find",
        description: "Find files by name",
        usage: "find [directory] <filename>",
        min_args: 1,
        max_args: Some(2),
    },
    CommandInfo {
        name: "sleep",
        description: "Sleep for specified seconds",
        usage: "sleep <seconds>",
        min_args: 1,
        max_args: Some(1),
    },
    CommandInfo {
        name: "version",
        description: "Show version information",
        usage: "version",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "jobs",
        description: "List background jobs",
        usage: "jobs",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "fg",
        description: "Show job status",
        usage: "fg [job_id]",
        min_args: 0,
        max_args: Some(1),
    },
    CommandInfo {
        name: "env",
        description: "Show or set environment variables",
        usage: "env [VAR[=value]]",
        min_args: 0,
        max_args: Some(1),
    },
    CommandInfo {
        name: "history",
        description: "Show command history or search",
        usage: "history [search <query>]",
        min_args: 0,
        max_args: None,
    },
];

impl Command {
    /// コマンド内の全ての変数を展開
    pub fn expand_variables(self) -> Self {
        match self {
            Command::Echo { message } => Command::Echo {
                message: expand_variables(&message),
            },
            Command::Cat { filename } => Command::Cat {
                filename: expand_variables(&filename),
            },
            Command::Write { filename, content } => Command::Write {
                filename: expand_variables(&filename),
                content: expand_variables(&content),
            },
            Command::Cd { path } => Command::Cd {
                path: expand_variables(&path),
            },
            Command::Mkdir { path, parents } => Command::Mkdir {
                path: expand_variables(&path),
                parents,
            },
            Command::Rm {
                path,
                recursive,
                force,
            } => Command::Rm {
                path: expand_variables(&path),
                recursive,
                force,
            },
            Command::Cp {
                source,
                destination,
                recursive,
            } => Command::Cp {
                source: expand_variables(&source),
                destination: expand_variables(&destination),
                recursive,
            },
            Command::Mv {
                source,
                destination,
            } => Command::Mv {
                source: expand_variables(&source),
                destination: expand_variables(&destination),
            },
            Command::Find { path, name } => Command::Find {
                path: path.map(|p| expand_variables(&p)),
                name: expand_variables(&name),
            },
            Command::Grep { pattern, files } => Command::Grep {
                pattern: expand_variables(&pattern),
                files: files.into_iter().map(|f| expand_variables(&f)).collect(),
            },
            Command::Alias { name, command } => Command::Alias {
                name: name.map(|n| expand_variables(&n)),
                command: command.map(|c| expand_variables(&c)),
            },
            Command::Repeat { count, message } => Command::Repeat {
                count,
                message: expand_variables(&message),
            },
            Command::FunctionCall { name, args } => Command::FunctionCall {
                name,
                args: args.into_iter().map(|arg| expand_variables(&arg)).collect(),
            },
            Command::Compound { commands } => Command::Compound {
                commands: commands
                    .into_iter()
                    .map(|cmd| cmd.expand_variables())
                    .collect(),
            },

            // 複合コマンドはそのまま（実行時に再度展開される）
            Command::If { .. } => self,
            Command::While { .. } => self,
            Command::For { .. } => self,
            Command::Pipeline { .. } => self,
            Command::Redirect { .. } => self,
            Command::Background { .. } => self,
            Command::Function { .. } => self,
            Command::History { .. } => self,

            // 変数を含まないコマンド
            Command::Help => self,
            Command::Version => self,
            Command::Pwd => self,
            Command::Ls => self,
            Command::Jobs => self,
            Command::Exit => self,
            Command::Sleep { .. } => self,
            Command::Fg { .. } => self,
            Command::Environment { .. } => self,
        }
    }
}

/// コマンドの実行
///
/// # Returns
/// * `Ok(true)` - プログラムを終了すべき場合
/// * `Ok(false)` - 実行を継続する場合
/// * `Err(...)` - エラーが発生した場合
pub fn execute_command(command: Command, input: Option<&str>) -> Result<bool> {
    match execute_command_internal(command, input)? {
        CommandResult::Continue(output) => {
            if !output.is_empty() {
                println!("{output}");
            }
            Ok(false)
        }
        CommandResult::Exit => Ok(true),
    }
}

/// execute_commandの内部処理
pub fn execute_command_internal(command: Command, input: Option<&str>) -> Result<CommandResult> {
    // コマンド実行開始を記録
    debug!("Executing command: {command:?}");

    let command = command.expand_variables();

    match command {
        Command::Help => Ok(CommandResult::Continue(handle_help())),
        Command::Cat { filename } => Ok(CommandResult::Continue(handle_cat(&filename, input)?)),
        Command::Echo { message } => Ok(CommandResult::Continue(handle_echo(&message))),
        Command::Write { filename, content } => {
            handle_write(&filename, &content)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Repeat { count, message } => {
            Ok(CommandResult::Continue(handle_repeat(count, &message)))
        }
        Command::Ls => Ok(CommandResult::Continue(handle_ls()?)),
        Command::Cd { path } => {
            handle_cd(&path)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Pwd => Ok(CommandResult::Continue(handle_pwd()?)),
        Command::Mkdir { path, parents } => {
            handle_mkdir(&path, parents)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Rm {
            path,
            recursive,
            force,
        } => {
            handle_rm(&path, recursive, force)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Cp {
            source,
            destination,
            recursive,
        } => {
            handle_cp(&source, &destination, recursive)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Mv {
            source,
            destination,
        } => {
            handle_mv(&source, &destination)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Find { path, name } => Ok(CommandResult::Continue(handle_find(
            path.as_deref(),
            &name,
        )?)),
        Command::Grep { pattern, files } => Ok(CommandResult::Continue(handle_grep(
            &pattern, &files, input,
        )?)),
        Command::Alias { name, command } => {
            handle_alias(name.as_deref(), command.as_deref())?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Version => Ok(CommandResult::Continue(handle_version())),
        Command::Pipeline { commands } => {
            let pipeline = PipelineCommand::new(commands);
            Ok(CommandResult::Continue(PipelineExecutor::execute(
                &pipeline,
            )?))
        }
        Command::Redirect {
            command,
            redirect_type,
            target,
        } => Ok(CommandResult::Continue(execute_redirect(
            *command,
            &redirect_type,
            &target,
        )?)),
        Command::Background { command } => Ok(CommandResult::Continue(
            handle_background_execution(command)?,
        )),
        Command::Sleep { seconds } => {
            handle_sleep(seconds)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Jobs => Ok(CommandResult::Continue(handle_jobs()?)),
        Command::Fg { job_id } => {
            handle_fg(job_id)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::Environment { action } => Ok(CommandResult::Continue(handle_environment(action)?)),
        Command::If {
            condition,
            then_part,
            else_part,
        } => {
            // conditionが成功すればthen,失敗すればelseパートを実行
            match execute_command(*condition, input) {
                Ok(should_exit) => {
                    if should_exit {
                        return Ok(CommandResult::Exit);
                    }
                    // thenの出力
                    if execute_command(*then_part, input)? {
                        return Ok(CommandResult::Exit);
                    }
                }
                Err(_) => {
                    if let Some(else_cmd) = else_part {
                        if execute_command(*else_cmd, input)? {
                            return Ok(CommandResult::Exit);
                        }
                    }
                }
            }
            Ok(CommandResult::Continue(String::new()))
        }
        Command::While { condition, body } => {
            let mut loop_count = 0;
            const MAX_ITERATIONS: usize = 1000;

            loop {
                if loop_count >= MAX_ITERATIONS {
                    return Err(crate::error::RucliError::RuntimeError(
                        "While loop exceeded maximum iterations".to_string(),
                    ));
                }

                // inputは無視してexecute_commandを使う
                match execute_command(*condition.clone(), None) {
                    Ok(should_exit) => {
                        if should_exit {
                            return Ok(CommandResult::Exit);
                        }
                        if execute_command(*body.clone(), None)? {
                            return Ok(CommandResult::Exit);
                        }
                    }
                    Err(_) => break,
                }

                loop_count += 1;
            }

            Ok(CommandResult::Continue(String::new()))
        }
        Command::For {
            variable,
            items,
            body,
        } => {
            for item in items {
                // ループ変数を環境変数として設定
                unsafe {
                    std::env::set_var(&variable, &item);
                }

                // bodyを実行
                if execute_command(*body.clone(), None)? {
                    unsafe {
                        std::env::remove_var(&variable);
                    }
                    return Ok(CommandResult::Exit);
                }
            }

            // ループ変数をクリア
            unsafe {
                std::env::remove_var(&variable);
            }

            Ok(CommandResult::Continue(String::new()))
        }
        Command::Function { name, body } => {
            handle_function_definition(&name, *body)?;
            Ok(CommandResult::Continue(String::new()))
        }
        Command::FunctionCall { name, args } => {
            Ok(CommandResult::Continue(handle_function_call(&name, &args)?))
        }
        Command::Compound { commands } => {
            for cmd in commands {
                if execute_command(cmd, input)? {
                    return Ok(CommandResult::Exit);
                }
            }
            Ok(CommandResult::Continue(String::new()))
        }
        Command::History { action } => match action {
            HistoryAction::List | HistoryAction::Search(_) => {
                Ok(CommandResult::Continue(handle_history(action)?))
            }
            HistoryAction::Execute(_) => {
                let cmd_str = handle_history(action)?;
                let cmd = parse_command(&cmd_str)?;
                execute_command_internal(cmd, input)
            }
        },
        Command::Exit => {
            handle_exit();
            Ok(CommandResult::Exit)
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
