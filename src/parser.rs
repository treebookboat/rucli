//! コマンドライン入力をパースするモジュール

use crate::commands::{COMMANDS, Command, CommandInfo};
use crate::error::{Result, RucliError};
use log::{debug, trace};

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

    match parts.as_slice() {
        ["help"] => Ok(Command::Help),
        ["echo", message @ ..] => Ok(Command::Echo {
            message: message.join(" "),
        }),
        ["cat", filename] => Ok(Command::Cat {
            filename: (*filename).to_string(),
        }),
        ["write", filename, content @ ..] => Ok(Command::Write {
            filename: (*filename).to_string(),
            content: content.join(" "),
        }),
        ["ls"] => Ok(Command::Ls),
        ["repeat", count, message @ ..] => match count.parse::<i32>() {
            Ok(count) if count > 0 => Ok(Command::Repeat {
                count,
                message: message.join(" "),
            }),
            Ok(_) => Err(RucliError::ParseError("count must be positive".to_string())),
            Err(_) => Err(RucliError::ParseError(format!(
                "{count} isn't a valid number"
            ))),
        },
        ["cd"] => Ok(Command::Cd {
            path: "~".to_string(),
        }),
        ["cd", path] => Ok(Command::Cd {
            path: (*path).to_string(),
        }),
        ["pwd"] => Ok(Command::Pwd),
        ["mkdir", "-p", path] => Ok(Command::Mkdir {
            path: (*path).to_string(),
            parents: true,
        }),
        ["mkdir", path] => Ok(Command::Mkdir {
            path: (*path).to_string(),
            parents: false,
        }),
        ["rm", "-r", path] => Ok(Command::Rm {
            path: (*path).to_string(),
            recursive: true,
            force: false,
        }),
        ["rm", "-f", path] => Ok(Command::Rm {
            path: (*path).to_string(),
            recursive: false,
            force: true,
        }),
        ["rm", "-rf", path] | ["rm", "-fr", path] => Ok(Command::Rm {
            path: (*path).to_string(),
            recursive: true,
            force: true,
        }),
        ["rm", path] => Ok(Command::Rm {
            path: (*path).to_string(),
            recursive: false,
            force: false,
        }),
        ["cp", source, destination] => Ok(Command::Cp {
            source: (*source).to_string(),
            destination: (*destination).to_string(),
        }),
        ["exit" | "quit"] => Ok(Command::Exit),
        commands => Err(RucliError::UnknownCommand(commands.join(" ").to_string())),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::{Command, CommandInfo},
        parser::{find_command, parse_command, validate_args},
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
}
