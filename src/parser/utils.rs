//! パーサーのユーティリティ関数と定数

use crate::commands::{COMMANDS, CommandInfo};
use crate::error::{Result, RucliError};
use log::{debug, trace};

pub const DEFAULT_HOME_INDICATOR: &str = "~";
pub const PREVIOUS_DIR_INDICATOR: &str = "-";

/// `コマンド名から対応するCommandInfo` を検索する
pub(super) fn find_command(name: &str) -> Option<&CommandInfo> {
    trace!("Looking for command: {name}");
    COMMANDS.iter().find(|command| command.name == name)
}

/// コマンドの引数数を検証する
pub(super) fn validate_args(cmd_info: &CommandInfo, args: &[&str]) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_validate_args_unlimited_max() {
        // max_argsがNone（無制限）のケース
        let cmd_info = CommandInfo {
            name: "echo",
            description: "Echo command",
            usage: "echo <message...>",
            min_args: 1,
            max_args: None,
        };

        // 多数の引数でもOK
        let args = vec!["arg1", "arg2", "arg3", "arg4", "arg5"];
        let result = validate_args(&cmd_info, &args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_args_exact_match() {
        // 最小と最大が同じ（固定引数）のケース
        let cmd_info = CommandInfo {
            name: "mv",
            description: "Move command",
            usage: "mv <source> <destination>",
            min_args: 2,
            max_args: Some(2),
        };

        // ちょうど2個
        let args = vec!["src", "dst"];
        assert!(validate_args(&cmd_info, &args).is_ok());

        // 1個（少ない）
        let args = vec!["src"];
        assert!(validate_args(&cmd_info, &args).is_err());

        // 3個（多い）
        let args = vec!["src", "dst", "extra"];
        assert!(validate_args(&cmd_info, &args).is_err());
    }
}
