//! コマンドライン入力をパースするモジュール

mod basic;
mod control;
mod file_ops;
mod operators;
mod utils;

// Re-export public items
pub use self::operators::{contains_heredoc, parse_heredoc_header, split_by_pipe};
pub use self::utils::{DEFAULT_HOME_INDICATOR, PREVIOUS_DIR_INDICATOR};

use crate::alias::get_alias;
use crate::commands::Command;
use crate::environment::expand_command_substitution;
use crate::error::{Result, RucliError};
use crate::functions;
use log::debug;

// Internal imports
use self::basic::*;
use self::control::*;
use self::file_ops::*;
use self::operators::*;
use self::utils::*;

/// ユーザー入力をコマンドに変換する
///
/// # Errors
///
/// - 空入力の場合
/// - 存在しないコマンドの場合
/// - 引数の数が不正な場合
pub fn parse_command(input: &str) -> Result<Command> {
    debug!("Parsing input: '{input}'");

    // コマンド置換を追加
    let substituted_input = expand_command_substitution(input)?;

    let input = substituted_input.as_str();

    let parts: Vec<&str> = input.split_whitespace().collect();

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

    // "&"があるかチェック
    if contains_background(input) {
        // "&"を除いた部分をパース
        let cmd_without_bg = input.trim_end().trim_end_matches('&').trim();
        let inner_cmd = parse_command(cmd_without_bg)?;

        return Ok(Command::Background {
            command: Box::new(inner_cmd),
        });
    }

    // ifのチェック
    if contains_if(input) {
        return parse_if_statement(input);
    }

    // whileのチェック
    if contains_while(input) {
        return parse_while_statement(input);
    }

    // forのチェック
    if contains_for(input) {
        return parse_for_statement(input);
    }

    // functionのチェック
    if contains_function(input) {
        return parse_function_definition(input);
    }

    // セミコロンを含むかチェック
    if split_by_semicolon(input).len() > 1 {
        return parse_multiple_commands(input);
    }

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
            for pipe_part in pipe_parts.iter().take(last_index) {
                commands.push(pipe_part.to_string());
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
        "jobs" => Ok(Command::Jobs),
        "history" => Ok(Command::History),
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
        "sleep" => parse_sleep(args),
        "fg" => parse_fg(args),
        "env" => parse_environment(args),

        _ => {
            if functions::is_function(cmd_name) {
                Ok(Command::FunctionCall {
                    name: cmd_name.to_string(),
                    args: args.iter().map(|s| s.to_string()).collect(),
                })
            } else {
                Err(RucliError::UnknownCommand(format!(
                    "{} {}",
                    cmd_name,
                    args.join(" ")
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_command_unknown() {
        // 不明なコマンドのエラー
        let result = parse_command("abc input");
        assert!(result.is_err())
    }

    #[test]
    fn test_parse_background_command() {
        // 基本的なバックグラウンドコマンド
        let result = parse_command("echo hello &");
        assert!(matches!(result, Ok(Command::Background { .. })));
    }

    #[test]
    fn test_parse_background_with_spaces() {
        // 前後に空白があるケース
        let result = parse_command("  echo hello & ");
        assert!(matches!(result, Ok(Command::Background { .. })));
    }

    #[test]
    fn test_parse_background_pipeline() {
        // パイプラインのバックグラウンド実行
        let result = parse_command("cat file.txt | grep pattern &");
        assert!(matches!(result, Ok(Command::Background { .. })));
    }
}
