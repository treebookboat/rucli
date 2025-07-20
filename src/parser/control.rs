//! 制御構造（if/while/for/function）のパース関数

use crate::commands::Command;
use crate::error::{Result, RucliError};
use crate::parser::{parse_command, split_by_semicolon};

// ifを含むかチェック
pub(super) fn contains_if(input: &str) -> bool {
    input.trim().starts_with("if ")
}

// whileを含むかチェック
pub(super) fn contains_while(input: &str) -> bool {
    input.trim().starts_with("while ")
}

// forを含むかチェック
pub(super) fn contains_for(input: &str) -> bool {
    input.trim().starts_with("for ")
}

/// 関数定義を含むかチェック
pub(super) fn contains_function(input: &str) -> bool {
    input.trim().starts_with("function ")
}

/// ifコマンドのパースを行う
pub(super) fn parse_if_statement(input: &str) -> Result<Command> {
    let input = input.trim();

    // 複数の空白を一つにまとめる
    let input = input.split_whitespace().collect::<Vec<_>>().join(" ");

    // thenの位置を探す
    let then_pos = input
        .find(" then ")
        .ok_or(RucliError::ParseError("if: 'then' not found".to_string()))?;

    // fiの位置を探す
    let fi_pos = input
        .rfind(" fi")
        .ok_or(RucliError::ParseError("if: 'fi' not found".to_string()))?;

    // else の位置を探す（オプション）
    let else_pos = input[then_pos..fi_pos]
        .find(" else ")
        .map(|pos| then_pos + pos);

    let condition_str = input["if ".len()..then_pos]
        .trim()
        .trim_end_matches(';') // 末尾のセミコロンを削除
        .trim();

    let (then_str, else_str) = if let Some(else_pos) = else_pos {
        let then_part = input[then_pos + " then ".len()..else_pos].trim();
        let else_part = input[else_pos + " else ".len()..fi_pos].trim();
        (then_part, Some(else_part))
    } else {
        let then_part = input[then_pos + " then ".len()..fi_pos].trim();
        (then_part, None)
    };

    // 各部分をパース
    let condition_cmd = parse_command(condition_str)?;
    let then_cmd = parse_multiple_commands(then_str)?;
    let else_cmd = else_str.map(|s| parse_multiple_commands(s)).transpose()?;

    Ok(Command::If {
        condition: Box::new(condition_cmd),
        then_part: Box::new(then_cmd),
        else_part: else_cmd.map(Box::new),
    })
}

/// whileコマンドのパースを行う
pub(super) fn parse_while_statement(input: &str) -> Result<Command> {
    let input = input.trim();

    // 複数の空白を一つにまとめる
    let input = input.split_whitespace().collect::<Vec<_>>().join(" ");

    // doの位置を探す
    let do_pos = input
        .find(" do ")
        .ok_or(RucliError::ParseError("while: 'do' not found".to_string()))?;

    // doneの位置を探す
    let done_pos = input.rfind(" done").ok_or(RucliError::ParseError(
        "while: 'done' not found".to_string(),
    ))?;

    let condition_str = input["while ".len()..do_pos]
        .trim()
        .trim_end_matches(';') // これを追加！
        .trim();
    let body_str = input[do_pos + " do ".len()..done_pos].trim();

    // 各部分をパース
    let condition_cmd = parse_command(condition_str)?;
    let body_cmd = parse_multiple_commands(body_str)?;

    Ok(Command::While {
        condition: Box::new(condition_cmd),
        body: Box::new(body_cmd),
    })
}

/// forコマンドのパースを行う
pub(super) fn parse_for_statement(input: &str) -> Result<Command> {
    let input = input.trim();

    // 複数の空白を一つにまとめる
    let input = input.split_whitespace().collect::<Vec<_>>().join(" ");

    // inの位置を探す
    let in_pos = input
        .find(" in ")
        .ok_or(RucliError::ParseError("for: 'in' not found".to_string()))?;

    // doの位置を探す
    let do_pos = input
        .find(" do ")
        .ok_or(RucliError::ParseError("for: 'do' not found".to_string()))?;

    // doneの位置を探す
    let done_pos = input
        .rfind(" done")
        .ok_or(RucliError::ParseError("for: 'done' not found".to_string()))?;

    // 各部分をパース
    let variable_str = input["for ".len()..in_pos].trim().to_string();
    let items_str = input[in_pos + " in ".len()..do_pos]
        .trim()
        .trim_end_matches(';') // 末尾のセミコロンを削除
        .trim();

    let items_vec = items_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let body_str = input[do_pos + " do ".len()..done_pos].trim();

    // bodyのパース
    let body_cmd = parse_multiple_commands(body_str)?;

    Ok(Command::For {
        variable: variable_str,
        items: items_vec,
        body: Box::new(body_cmd),
    })
}

/// 関数定義をパースする
///
/// # Arguments
/// * `input` - "function name() { body }" 形式の文字列
///
/// # Returns
/// * `Ok(Command::Function)` - パース成功
/// * `Err(RucliError)` - パース失敗
///
pub(super) fn parse_function_definition(input: &str) -> Result<Command> {
    let input = input.trim();

    // 複数の空白を一つにまとめる
    let input = input.split_whitespace().collect::<Vec<_>>().join(" ");

    // (の位置を探す
    let start_parens_pos = input.find("(").ok_or(RucliError::ParseError(
        "function: '(' not found".to_string(),
    ))?;

    // )の位置も確認（(の後に)があるか）
    let end_parens_pos = input.find(")").ok_or(RucliError::ParseError(
        "function: ')' not found".to_string(),
    ))?;

    // ()の間が空であることを確認（引数は未対応）
    if end_parens_pos - start_parens_pos != 1 {
        return Err(RucliError::ParseError(
            "function: parameters not supported".to_string(),
        ));
    }

    // {の位置を探す
    let start_bracket_pos = input.find("{").ok_or(RucliError::ParseError(
        "function: '{' not found".to_string(),
    ))?;

    // }の位置を探す
    let end_bracket_pos = input.find("}").ok_or(RucliError::ParseError(
        "function: '}' not found".to_string(),
    ))?;

    // 関数名を取得
    let name_str = input["function ".len()..start_parens_pos].trim();

    // 処理部分を取得
    let body_str = input[start_bracket_pos + 1..end_bracket_pos].trim();

    // bodyのパース
    let body_cmd = parse_multiple_commands(body_str)?;

    Ok(Command::Function {
        name: name_str.to_string(),
        body: Box::new(body_cmd),
    })
}

/// 複数のコマンドをパースする
pub(super) fn parse_multiple_commands(input: &str) -> Result<Command> {
    // 入力の分割を行う
    let split_str = split_by_semicolon(input);

    // 命令が一つであればそれを返す
    if split_str.len() == 1 {
        parse_command(split_str[0])
    }
    // 複数の命令があればそれらすべてをパースする
    else {
        let mut commands = Vec::new();
        for cmd_str in split_str {
            commands.push(parse_command(cmd_str)?);
        }
        Ok(Command::Compound { commands })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functions;

    #[test]
    fn test_contains_if() {
        assert!(contains_if("if echo test; then echo OK; fi"));
        assert!(contains_if("  if cat file; then echo found; fi"));
        assert!(!contains_if("echo if test"));
        assert!(!contains_if("gift"));
    }

    #[test]
    fn test_parse_if_basic() {
        let input = "if echo test; then echo success; fi";
        let cmd = parse_if_statement(input).unwrap();

        match cmd {
            Command::If {
                condition,
                then_part,
                else_part,
            } => {
                // conditionがEchoコマンドであることを確認
                assert!(matches!(*condition, Command::Echo { .. }));
                // then_partもEchoコマンド
                assert!(matches!(*then_part, Command::Echo { .. }));
                // else_partはNone
                assert!(else_part.is_none());
            }
            _ => panic!("Expected If command"),
        }
    }

    #[test]
    fn test_parse_if_with_else() {
        let input = "if cat nonexistent; then echo found; else echo not found; fi";
        let cmd = parse_if_statement(input).unwrap();

        match cmd {
            Command::If {
                condition,
                then_part,
                else_part,
            } => {
                assert!(matches!(*condition, Command::Cat { .. }));
                assert!(matches!(*then_part, Command::Echo { .. }));
                assert!(else_part.is_some());
                assert!(matches!(*else_part.unwrap(), Command::Echo { .. }));
            }
            _ => panic!("Expected If command"),
        }
    }

    #[test]
    fn test_parse_if_missing_then() {
        let input = "if echo test; echo OK; fi";
        let result = parse_if_statement(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("then"));
    }

    #[test]
    fn test_parse_if_missing_fi() {
        let input = "if echo test; then echo OK";
        let result = parse_if_statement(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("fi"));
    }

    #[test]
    fn test_contains_while() {
        assert!(contains_while("while echo test; do echo OK; done"));
        assert!(contains_while("  while cat file; do rm file; done"));
        assert!(!contains_while("echo while test"));
        assert!(!contains_while("meanwhile"));
    }

    #[test]
    fn test_parse_while_basic() {
        let input = "while echo testing; do echo loop; done";
        let cmd = parse_while_statement(input).unwrap();

        match cmd {
            Command::While { condition, body } => {
                // conditionがEchoコマンドであることを確認
                assert!(matches!(*condition, Command::Echo { .. }));
                // bodyもEchoコマンド
                assert!(matches!(*body, Command::Echo { .. }));
            }
            _ => panic!("Expected While command"),
        }
    }

    #[test]
    fn test_parse_while_with_cat() {
        let input = "while cat file.txt; do rm file.txt; done";
        let cmd = parse_while_statement(input).unwrap();

        match cmd {
            Command::While { condition, body } => {
                assert!(matches!(*condition, Command::Cat { .. }));
                assert!(matches!(*body, Command::Rm { .. }));
            }
            _ => panic!("Expected While command"),
        }
    }

    #[test]
    fn test_parse_while_missing_do() {
        let input = "while echo test; echo loop; done";
        let result = parse_while_statement(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("do"));
    }

    #[test]
    fn test_parse_while_missing_done() {
        let input = "while echo test; do echo loop";
        let result = parse_while_statement(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("done"));
    }

    #[test]
    fn test_contains_for() {
        assert!(contains_for("for i in 1 2 3; do echo $i; done"));
        assert!(contains_for("  for name in Alice Bob; do echo $name; done"));
        assert!(!contains_for("echo for test"));
        assert!(!contains_for("fortune"));
    }

    #[test]
    fn test_parse_for_basic() {
        let input = "for i in 1 2 3; do echo $i; done";
        let cmd = parse_for_statement(input).unwrap();

        match cmd {
            Command::For {
                variable,
                items,
                body,
            } => {
                assert_eq!(variable, "i");
                assert_eq!(items, vec!["1", "2", "3"]);
                assert!(matches!(*body, Command::Echo { .. }));
            }
            _ => panic!("Expected For command"),
        }
    }

    #[test]
    fn test_parse_for_with_names() {
        let input = "for name in Alice Bob Charlie; do echo Hello $name; done";
        let cmd = parse_for_statement(input).unwrap();

        match cmd {
            Command::For {
                variable,
                items,
                body,
            } => {
                assert_eq!(variable, "name");
                assert_eq!(items, vec!["Alice", "Bob", "Charlie"]);
                assert!(matches!(*body, Command::Echo { .. }));
            }
            _ => panic!("Expected For command"),
        }
    }

    #[test]
    fn test_parse_for_missing_in() {
        let input = "for i 1 2 3; do echo $i; done";
        let result = parse_for_statement(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("in"));
    }

    #[test]
    fn test_parse_for_missing_do() {
        let input = "for i in 1 2 3; echo $i; done";
        let result = parse_for_statement(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("do"));
    }

    #[test]
    fn test_parse_for_missing_done() {
        let input = "for i in 1 2 3; do echo $i";
        let result = parse_for_statement(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("done"));
    }

    #[test]
    fn test_parse_function_simple() {
        let input = "function greet() { echo Hello }";
        let result = parse_function_definition(input).unwrap();

        match result {
            Command::Function { name, body } => {
                assert_eq!(name, "greet");
                assert!(matches!(*body, Command::Echo { .. }));
            }
            _ => panic!("Expected Function command"),
        }
    }

    #[test]
    fn test_parse_function_with_semicolon() {
        let input = "function test() { echo Test; }";
        let result = parse_function_definition(input).unwrap();
        assert!(matches!(result, Command::Function { .. }));
    }

    #[test]
    fn test_parse_function_missing_parentheses() {
        let input = "function test { echo Hello }";
        let result = parse_function_definition(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_function_missing_braces() {
        let input = "function test() echo Hello";
        let result = parse_function_definition(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_function_call() {
        // 関数を定義
        let body = Command::Echo {
            message: "test".to_string(),
        };
        functions::define_function("mytest", body);

        // 関数呼び出しをパース
        let result = parse_command("mytest arg1 arg2").unwrap();

        match result {
            Command::FunctionCall { name, args } => {
                assert_eq!(name, "mytest");
                assert_eq!(args, vec!["arg1", "arg2"]);
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_multiple_commands_single() {
        let cmd = parse_multiple_commands("echo single").unwrap();
        assert!(matches!(cmd, Command::Echo { .. }));
    }

    #[test]
    fn test_parse_multiple_commands_multiple() {
        let cmd = parse_multiple_commands("echo a; echo b").unwrap();
        match cmd {
            Command::Compound { commands } => {
                assert_eq!(commands.len(), 2);
            }
            _ => panic!("Expected Compound command"),
        }
    }

    #[test]
    fn test_if_with_multiple_commands() {
        let cmd =
            parse_if_statement("if echo condition; then echo first; echo second; fi").unwrap();
        match cmd {
            Command::If { then_part, .. } => {
                assert!(matches!(*then_part, Command::Compound { .. }));
            }
            _ => panic!("Expected If command"),
        }
    }

    #[test]
    fn test_for_with_multiple_commands() {
        let cmd = parse_for_statement("for i in 1 2; do echo Number:; echo $i; done").unwrap();
        match cmd {
            Command::For { body, .. } => {
                assert!(matches!(*body, Command::Compound { .. }));
            }
            _ => panic!("Expected For command"),
        }
    }
}
