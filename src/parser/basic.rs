//! 基本コマンドのパース関数

use crate::commands::{Command, EnvironmentAction};
use crate::error::{Result, RucliError};
use crate::parser::utils::DEFAULT_HOME_INDICATOR;

pub(super) fn parse_echo(args: &[&str]) -> Result<Command> {
    Ok(Command::Echo {
        message: args.join(" "),
    })
}

pub(super) fn parse_cat(args: &[&str]) -> Result<Command> {
    if args.is_empty() {
        // 引数なしの場合は、標準入力から読むことを想定
        // ダミーのファイル名を使う（実際には使われない）
        Ok(Command::Cat {
            filename: String::new(),
        })
    } else {
        Ok(Command::Cat {
            filename: args[0].to_string(),
        })
    }
}

pub(super) fn parse_write(args: &[&str]) -> Result<Command> {
    Ok(Command::Write {
        filename: args[0].to_string(),
        content: args[1..].join(" "),
    })
}

pub(super) fn parse_repeat(args: &[&str]) -> Result<Command> {
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

pub(super) fn parse_cd(args: &[&str]) -> Result<Command> {
    Ok(Command::Cd {
        path: args
            .first()
            .map(|s| s.to_string())
            .unwrap_or_else(|| DEFAULT_HOME_INDICATOR.to_string()),
    })
}

pub(super) fn parse_sleep(args: &[&str]) -> Result<Command> {
    match args[0].parse::<u64>() {
        Ok(seconds) => Ok(Command::Sleep { seconds }),
        Err(_) => Err(RucliError::ParseError(format!(
            "'{}' is not a valid number",
            args[0]
        ))),
    }
}

pub(super) fn parse_alias(args: &[&str]) -> Result<Command> {
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

pub(super) fn parse_fg(args: &[&str]) -> Result<Command> {
    match args {
        [] => Ok(Command::Fg { job_id: None }),
        [job_id] => match job_id.parse::<u32>() {
            Ok(job_id) => Ok(Command::Fg {
                job_id: Some(job_id),
            }),
            Err(_) => Err(RucliError::ParseError(format!(
                "'{}' is not a valid number",
                args[0]
            ))),
        },
        _ => unreachable!(),
    }
}

/// envコマンドのパース関数
pub(super) fn parse_environment(args: &[&str]) -> Result<Command> {
    // 処理パターン:
    // [] => List (引数なし)
    // ["VAR"] => Show(VAR)
    // ["VAR=value"] => Set(VAR, value)

    match args {
        [] => Ok(Command::Environment {
            action: EnvironmentAction::List,
        }),
        [var] => {
            if let Some((name, value)) = var.split_once("=") {
                Ok(Command::Environment {
                    action: EnvironmentAction::Set(name.to_string(), value.to_string()),
                })
            } else {
                Ok(Command::Environment {
                    action: EnvironmentAction::Show(var.to_string()),
                })
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_echo() {
        // echoコマンドのパース成功
        let result = parse_echo(&["input"]);
        assert!(matches!(result,Ok(Command::Echo{message}) if message == "input"))
    }

    #[test]
    fn test_parse_echo_multiple_words() {
        let result = parse_echo(&["hello", "world"]);
        assert!(matches!(result, Ok(Command::Echo { message }) if message == "hello world"));
    }

    #[test]
    fn test_parse_cat_with_file() {
        let result = parse_cat(&["test.txt"]);
        assert!(matches!(result, Ok(Command::Cat { filename }) if filename == "test.txt"));
    }

    #[test]
    fn test_parse_cat_no_args() {
        let result = parse_cat(&[]);
        assert!(matches!(result, Ok(Command::Cat { filename }) if filename.is_empty()));
    }

    #[test]
    fn test_parse_write() {
        let result = parse_write(&["file.txt", "hello", "world"]);
        match result {
            Ok(Command::Write { filename, content }) => {
                assert_eq!(filename, "file.txt");
                assert_eq!(content, "hello world");
            }
            _ => panic!("Expected Write command"),
        }
    }

    #[test]
    fn test_parse_repeat_valid() {
        let result = parse_repeat(&["3", "hello"]);
        assert!(matches!(result, Ok(Command::Repeat { count: 3, message }) if message == "hello"));
    }

    #[test]
    fn test_parse_repeat_invalid_count() {
        let result = parse_repeat(&["-1", "test"]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be positive"));

        let result2 = parse_repeat(&["abc", "test"]);
        assert!(result2.is_err());
        assert!(result2.unwrap_err().to_string().contains("isn't a valid number"));
    }

    #[test]
    fn test_parse_cd_with_path() {
        let result = parse_cd(&["/home/user"]);
        assert!(matches!(result, Ok(Command::Cd { path }) if path == "/home/user"));
    }

    #[test]
    fn test_parse_cd_no_args() {
        let result = parse_cd(&[]);
        assert!(matches!(result, Ok(Command::Cd { path }) if path == DEFAULT_HOME_INDICATOR));
    }

    #[test]
    fn test_parse_sleep_valid() {
        let result = parse_sleep(&["5"]);
        assert!(matches!(result, Ok(Command::Sleep { seconds: 5 })));
    }

    #[test]
    fn test_parse_sleep_invalid_arg() {
        let result = parse_sleep(&["abc"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_alias_no_args() {
        let result = parse_alias(&[]);
        assert!(matches!(result, Ok(Command::Alias { name: None, command: None })));
    }

    #[test]
    fn test_parse_alias_with_setting() {
        let result = parse_alias(&["ll=ls"]);
        match result {
            Ok(Command::Alias { name, command }) => {
                assert_eq!(name, Some("ll".to_string()));
                assert_eq!(command, Some("ls".to_string()));
            }
            _ => panic!("Expected Alias command"),
        }
    }

    #[test]
    fn test_parse_alias_invalid() {
        let result = parse_alias(&["invalid"]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("alias needs ="));
    }

    #[test]
    fn test_parse_fg_no_args() {
        let result = parse_fg(&[]);
        assert!(matches!(result, Ok(Command::Fg { job_id: None })));
    }

    #[test]
    fn test_parse_fg_with_job_id() {
        let result = parse_fg(&["1"]);
        assert!(matches!(result, Ok(Command::Fg { job_id: Some(1) })));
    }

    #[test]
    fn test_parse_fg_invalid_job_id() {
        let result = parse_fg(&["abc"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_env_command_list() {
        let result = parse_environment(&[]);
        assert!(matches!(result, Ok(Command::Environment { action: EnvironmentAction::List })));
    }

    #[test]
    fn test_parse_env_command_set() {
        let result = parse_environment(&["TEST_VAR=test_value"]);
        assert!(matches!(result, Ok(Command::Environment { 
            action: EnvironmentAction::Set(var, val) 
        }) if var == "TEST_VAR" && val == "test_value"));
    }

    #[test]
    fn test_parse_env_command_show() {
        let result = parse_environment(&["PATH"]);
        assert!(matches!(result, Ok(Command::Environment { 
            action: EnvironmentAction::Show(var) 
        }) if var == "PATH"));
    }
}