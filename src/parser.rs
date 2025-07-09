//! コマンドライン入力をパースするモジュール

use crate::alias::get_alias;
use crate::commands::{COMMANDS, Command, CommandInfo};
use crate::error::{Result, RucliError};
use log::{debug, trace};

pub const DEFAULT_HOME_INDICATOR: &str = "~";
pub const PREVIOUS_DIR_INDICATOR: &str = "-";

/// `コマンド名から対応するCommandInfo` を検索する
fn find_command(name: &str) -> Option<&CommandInfo> {
    trace!("Looking for command: {name}");
    COMMANDS.iter().find(|command| command.name == name)
}

/// コマンドの引数数を検証する
fn validate_args(cmd_info: &CommandInfo, args: &[&str]) -> Result<()> {
    debug!(
        "Validating args for '{}': {} args provided",
        cmd_info.name,
        args.len()
    );

    // 引数の個数
    let arg_count = args.len();

    // 最小引数チェック
    if arg_count < cmd_info.min_args {
        return Err(RucliError::InvalidArgument(
            [
                format!(
                    "{} requires at least {} argument(s)",
                    cmd_info.name, cmd_info.min_args
                ),
                format!("Usage: {}", cmd_info.usage),
            ]
            .join("\n"),
        ));
    }

    // 最大引数チェック
    if let Some(max) = cmd_info.max_args
        && max < arg_count
    {
        return Err(RucliError::InvalidArgument(
            [
                format!("{} accepts at most {} argument(s)", cmd_info.name, max),
                format!("Usage: {}", cmd_info.usage),
            ]
            .join("\n"),
        ));
    }

    // ここまでくればOK
    Ok(())
}

/// ユーザー入力をコマンドに変換する
///
/// # Errors
///
/// - 空入力の場合
/// - 存在しないコマンドの場合
/// - 引数の数が不正な場合
/// - repeatコマンドで数値以外や負の数を指定した場合
pub fn parse_command(input: &str) -> Result<Command> {
    debug!("Parsing input: '{input}'");

    let parts: Vec<&str> = input.split_whitespace().collect();

    // 空入力チェック
    if parts.is_empty() {
        debug!("Empty input detected");
        return Err(RucliError::ParseError("No command provided".to_string()));
    }

    let cmd_name = parts[0];
    let args = &parts[1..];

    // エイリアス展開部分
    let alias = get_alias(cmd_name);
    let cmd_name = if cmd_name != "alias" {
        debug!("alias succeed");
        alias.as_deref().unwrap_or(cmd_name)
    } else {
        debug!("no alias");
        cmd_name
    };

    // まずパイプで分割
    let pipe_parts = split_by_pipe(input);

    if pipe_parts.len() > 1 {
        let mut commands = Vec::new();
        let last_index = pipe_parts.len() - 1;

        // 最後のコマンドを特別処理
        let last_part = pipe_parts[last_index];
        if contains_redirect(last_part) {
            let (cmd_str, redirect_opt) = split_redirect(last_part);

            // 最後以外のコマンドを追加
            for i in 0..last_index {
                commands.push(pipe_parts[i].to_string());
            }
            // 最後のコマンド（リダイレクトなし）を追加
            commands.push(cmd_str);

            if let Some((redirect_type, target)) = redirect_opt {
                // パイプライン全体をリダイレクト
                return Ok(Command::Redirect {
                    command: Box::new(Command::Pipeline { commands }),
                    redirect_type,
                    target,
                });
            }
        } else {
            // リダイレクトなしの通常のパイプライン
            for part in pipe_parts {
                commands.push(part.to_string());
            }
        }

        return Ok(Command::Pipeline { commands });
    }

    // リダイレクトのみ
    if contains_redirect(input) {
        let (cmd_str, redirect_opt) = split_redirect(input);

        if let Some((redirect_type, target)) = redirect_opt {
            let inner_command = parse_command(&cmd_str)?;

            return Ok(Command::Redirect {
                command: Box::new(inner_command),
                redirect_type,
                target,
            });
        }
    }

    // パイプラインチェックを追加
    if contains_pipeline(input) {
        let commands = split_by_pipe(input).iter().map(|s| s.to_string()).collect();

        return Ok(Command::Pipeline { commands });
    }

    // 引数の数チェック
    if let Some(cmd_info) = find_command(cmd_name) {
        validate_args(cmd_info, args)?;
    }

    // コマンド認識時
    debug!(
        "Recognized command: '{}' with {} args",
        cmd_name,
        args.len()
    );

    match cmd_name {
        "help" => Ok(Command::Help),
        "version" => Ok(Command::Version),
        "pwd" => Ok(Command::Pwd),
        "ls" => Ok(Command::Ls),
        "exit" | "quit" => Ok(Command::Exit),

        "echo" => parse_echo(args),
        "cat" => parse_cat(args),
        "write" => parse_write(args),
        "repeat" => parse_repeat(args),
        "cd" => parse_cd(args),
        "mkdir" => parse_mkdir(args),
        "rm" => parse_rm(args),
        "cp" => parse_cp(args),
        "mv" => parse_mv(args),
        "find" => parse_find(args),
        "grep" => parse_grep(args),
        "alias" => parse_alias(args),

        _ => Err(RucliError::UnknownCommand(format!(
            "{} {}",
            cmd_name,
            args.join(" ")
        ))),
    }
}

fn parse_echo(args: &[&str]) -> Result<Command> {
    Ok(Command::Echo {
        message: args.join(" "),
    })
}

fn parse_cat(args: &[&str]) -> Result<Command> {
    Ok(Command::Cat {
        filename: args[0].to_string(),
    })
}

fn parse_write(args: &[&str]) -> Result<Command> {
    Ok(Command::Write {
        filename: args[0].to_string(),
        content: args[1..].join(" "),
    })
}

fn parse_repeat(args: &[&str]) -> Result<Command> {
    match args[0].parse::<i32>() {
        Ok(count) if count > 0 => Ok(Command::Repeat {
            count,
            message: args[1..].join(" "),
        }),
        Ok(_) => Err(RucliError::ParseError("count must be positive".to_string())),
        Err(_) => Err(RucliError::ParseError(format!(
            "{} isn't a valid number",
            args[0]
        ))),
    }
}

fn parse_cd(args: &[&str]) -> Result<Command> {
    Ok(Command::Cd {
        path: args
            .first()
            .map(|s| s.to_string())
            .unwrap_or_else(|| DEFAULT_HOME_INDICATOR.to_string()),
    })
}

fn parse_mkdir(args: &[&str]) -> Result<Command> {
    match args {
        ["-p", path] => Ok(Command::Mkdir {
            path: path.to_string(),
            parents: true,
        }),
        [path] => Ok(Command::Mkdir {
            path: path.to_string(),
            parents: false,
        }),
        _ => unreachable!(),
    }
}

fn parse_rm(args: &[&str]) -> Result<Command> {
    match args {
        ["-r", path] => Ok(Command::Rm {
            path: path.to_string(),
            recursive: true,
            force: false,
        }),
        ["-f", path] => Ok(Command::Rm {
            path: path.to_string(),
            recursive: false,
            force: true,
        }),
        ["-rf", path] | ["-fr", path] => Ok(Command::Rm {
            path: path.to_string(),
            recursive: true,
            force: true,
        }),
        [path] => Ok(Command::Rm {
            path: path.to_string(),
            recursive: false,
            force: false,
        }),
        _ => unreachable!(),
    }
}

fn parse_cp(args: &[&str]) -> Result<Command> {
    match args {
        ["-r", src, dst] => Ok(Command::Cp {
            source: src.to_string(),
            destination: dst.to_string(),
            recursive: true,
        }),
        [src, dst] => Ok(Command::Cp {
            source: src.to_string(),
            destination: dst.to_string(),
            recursive: false,
        }),
        _ => unreachable!(),
    }
}

fn parse_mv(args: &[&str]) -> Result<Command> {
    Ok(Command::Mv {
        source: args[0].to_string(),
        destination: args[1].to_string(),
    })
}

fn parse_find(args: &[&str]) -> Result<Command> {
    match args.len() {
        1 => Ok(Command::Find {
            path: None,
            name: args[0].to_string(),
        }),
        2 => Ok(Command::Find {
            path: Some(args[0].to_string()),
            name: args[1].to_string(),
        }),
        _ => unreachable!(),
    }
}

fn parse_grep(args: &[&str]) -> Result<Command> {
    Ok(Command::Grep {
        pattern: args[0].to_string(),
        files: args[1..].iter().map(|f| f.to_string()).collect(),
    })
}

fn parse_alias(args: &[&str]) -> Result<Command> {
    match args {
        [] => Ok(Command::Alias {
            name: None,
            command: None,
        }),
        [setting] => match setting.split_once("=") {
            Some((name, cmd)) => Ok(Command::Alias {
                name: Some(name.to_string()),
                command: Some(cmd.to_string()),
            }),
            None => Err(RucliError::ParseError("alias needs =".to_string())),
        },
        _ => unreachable!(),
    }
}

/// 入力をパイプで分割する
/// 例: "echo hello | grep h" → ["echo hello", "grep h"]
pub fn split_by_pipe(input: &str) -> Vec<&str> {
    input
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect()
}

/// パイプラインを含むかチェック
pub fn contains_pipeline(input: &str) -> bool {
    input.contains('|')
}

// リダイレクトでコマンドを分割
/// 例: "echo hello > file.txt" → ("echo hello", Some((">", "file.txt")))
pub fn split_redirect(input: &str) -> (String, Option<(String, String)>) {
    if let Some(pos) = input.find('>') {
        let command = input[..pos].trim().to_string();
        let target = input[pos + 1..].trim().to_string();

        if target.is_empty() {
            (input.to_string(), None)
        } else {
            (command, Some((">".to_string(), target)))
        }
    } else {
        (input.to_string(), None)
    }
}

/// リダイレクトを含むかチェック
pub fn contains_redirect(input: &str) -> bool {
    input.contains(">")
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::{Command, CommandInfo},
        parser::{contains_pipeline, find_command, parse_command, split_by_pipe, validate_args},
    };

    #[test]
    fn test_find_command_exists() {
        // "echo" コマンドが見つかることを確認
        let result = find_command("echo");

        assert!(matches!(result, Some(cmd) if cmd.name == "echo"))
    }

    #[test]
    fn test_find_command_not_exists() {
        // 存在しないコマンドでNoneが返ることを確認
        let result = find_command("abc");

        assert!(result.is_none())
    }

    #[test]
    fn test_validate_args_min_args() {
        // 最小引数のエラーをテスト

        // テスト用のCommandInfoを作成
        let cmd_info = CommandInfo {
            name: "test_cmd",
            description: "Test command",
            usage: "test_cmd <arg1> <arg2>",
            min_args: 2,
            max_args: None,
        };

        // 引数が足りないケース
        let args = vec!["arg1"]; // 1個だけ（2個必要）

        let result = validate_args(&cmd_info, &args);

        assert!(result.is_err())
    }

    #[test]
    fn test_validate_args_max_args() {
        // 最大引数のエラーをテスト

        // テスト用のCommandInfoを作成
        let cmd_info = CommandInfo {
            name: "test_cmd",
            description: "Test command",
            usage: "test_cmd <arg1> <arg2>",
            min_args: 2,
            max_args: Some(3),
        };

        // 引数が足りないケース
        let args = vec!["arg1", "arg2", "arg3", "arg4"]; // 4個（3個まで）

        let result = validate_args(&cmd_info, &args);

        assert!(result.is_err())
    }

    #[test]
    fn test_validate_args_success() {
        // 正常なケースをテスト

        // テスト用のCommandInfoを作成
        let cmd_info = CommandInfo {
            name: "test_cmd",
            description: "Test command",
            usage: "test_cmd <arg1> <arg2>",
            min_args: 2,
            max_args: Some(3),
        };

        // 引数が足りないケース
        let args = vec!["arg1", "arg2"]; // 1個だけ（2個必要）

        let result = validate_args(&cmd_info, &args);

        assert!(result.is_ok())
    }

    #[test]
    fn test_parse_command_empty_input() {
        // 空入力のテスト
        let result = parse_command("");

        assert!(result.is_err())
    }

    #[test]
    fn test_parse_command_echo() {
        // echoコマンドのパース成功

        let result = parse_command("echo input");

        assert!(matches!(result,Ok(Command::Echo{message}) if message == "input"))
    }

    #[test]
    fn test_parse_command_unknown() {
        // 不明なコマンドのエラー
        let result = parse_command("abc input");

        assert!(result.is_err())
    }

    #[test]
    fn test_split_by_pipe() {
        let input = "echo hello | grep h | wc -l";
        let parts = split_by_pipe(input);
        assert_eq!(parts, vec!["echo hello", "grep h", "wc -l"]);
    }

    #[test]
    fn test_split_by_pipe_with_spaces() {
        let input = "  echo hello  |  grep h  ";
        let parts = split_by_pipe(input);
        assert_eq!(parts, vec!["echo hello", "grep h"]);
    }

    #[test]
    fn test_contains_pipeline() {
        assert!(contains_pipeline("echo | grep"));
        assert!(!contains_pipeline("echo hello"));
    }
}
