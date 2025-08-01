use crate::error::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::commands::{CommandResult, execute_command_internal};
use crate::parser::parse_command;

/// セッション固有の環境変数ストレージ
static SESSION_VARS: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 環境変数を取得
pub fn get_var(name: &str) -> Option<String> {
    // SESSION_VARSをロックして取得
    let session_vars = SESSION_VARS.lock().unwrap();

    // セッション変数から検索
    if let Some(value) = session_vars.get(name) {
        return Some(value.clone());
    }

    // セッション変数になければシステム変数から検索
    std::env::var(name).ok()
}

/// 環境変数を設定
pub fn set_var(name: &str, value: &str) {
    // SESSION_VARSをロックして取得
    let mut session_vars = SESSION_VARS.lock().unwrap();
    session_vars.insert(name.to_string(), value.to_string());
}

// 環境変数をすべて表示
pub fn list_all_vars() -> Vec<(String, String)> {
    let mut result = Vec::new();

    // システム環境変数を取得
    for (key, value) in std::env::vars() {
        result.push((key, value));
    }

    // セッション変数を取得
    let session_vars = SESSION_VARS.lock().unwrap();

    for (key, value) in session_vars.iter() {
        // 既存のシステム変数があれば上書き、なければ追加
        if let Some(pos) = result.iter().position(|(k, _)| k == key) {
            result[pos] = (key.clone(), value.clone());
        } else {
            result.push((key.clone(), value.clone()));
        }
    }

    result
}

/// 変数展開を行う関数
pub fn expand_variables(input: &str) -> String {
    // 結果を格納する文字列
    let mut ans_string = String::new();
    let mut chars = input.chars().peekable();

    // 文字列をスキャンして$以降の単語を置換
    while let Some(char) = chars.next() {
        if char == '$' {
            if chars.peek() == Some(&'{') {
                // '{'部分を進める
                chars.next();

                let mut var_name = String::new();
                let mut found_closing_brace = false;

                // '}'を取得できるまで進める
                while let Some(&next_char) = chars.peek() {
                    // 変数名に使える文字なら追加
                    if next_char != '}' {
                        var_name.push(chars.next().unwrap());
                    } else {
                        chars.next();
                        found_closing_brace = true;
                        break;
                    }
                }

                // 変数名が取得できた場合は置換
                if found_closing_brace && !var_name.is_empty() {
                    if let Some(value) = get_var(&var_name) {
                        ans_string.push_str(&value);
                    }
                }
                // 空文字列はそのまま出力
                else if found_closing_brace && var_name.is_empty() {
                    ans_string.push_str("${}");
                }
                // 閉じカッコが存在しない場合も元の文字列をそのまま出力
                else {
                    ans_string.push_str("${");
                    ans_string.push_str(&var_name);
                }
            } else {
                let mut var_name = String::new();
                while let Some(&next_char) = chars.peek() {
                    // 変数名に使える文字なら追加
                    if next_char.is_alphanumeric() || next_char == '_' {
                        var_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if !var_name.is_empty() {
                    // 変数名が取得できた場合は置換
                    if let Some(value) = get_var(&var_name) {
                        ans_string.push_str(&value);
                    }
                } else {
                    ans_string.push('$');
                }
            }
        } else {
            ans_string.push(char);
        }
    }

    ans_string
}

/// コマンド置換を実行する関数
pub fn expand_command_substitution(input: &str) -> Result<String> {
    // 結果を格納する文字列
    let mut ans_string = String::new();
    let mut chars = input.chars().peekable();

    // 文字列をスキャンして$以降の単語を置換
    while let Some(ch) = chars.next() {
        if ch == '$' {
            // 次の文字が(かチェック
            if chars.peek() == Some(&'(') {
                // (を消費
                chars.next();

                let mut paren_count = 1;
                let mut cmd_string = String::new();
                let mut found_closing_brace = false;

                // )まで読み取る
                for next_ch in chars.by_ref() {
                    if next_ch == '(' {
                        paren_count += 1;
                        cmd_string.push(next_ch);
                    } else if next_ch == ')' {
                        paren_count -= 1;
                        if paren_count == 0 {
                            found_closing_brace = true;
                            break;
                        }
                        cmd_string.push(next_ch);
                    } else {
                        cmd_string.push(next_ch);
                    }
                }

                // 変数名が取得できた場合は置換
                if found_closing_brace && !cmd_string.is_empty() {
                    // 再帰的に内部のコマンド置換を実行
                    let inner_expanded = expand_command_substitution(&cmd_string)?;

                    match parse_command(&inner_expanded) {
                        Ok(cmd) => {
                            match execute_command_internal(cmd, None) {
                                Ok(CommandResult::Continue(output)) => {
                                    // 末尾の改行を削除
                                    ans_string.push_str(output.trim_end());
                                }
                                Ok(CommandResult::Exit) => {
                                    // コマンド置換内でのExitは無視
                                }
                                Err(_) => {
                                    // エラーなのでなにもしない
                                }
                            }
                        }
                        Err(_) => {
                            // パースエラーなのでなにもしない
                        }
                    }
                }
                // 空文字列はそのまま出力
                else if found_closing_brace && cmd_string.is_empty() {
                    // 何もしない
                }
                // 閉じカッコが存在しない場合も元の文字列をそのまま出力
                else {
                    ans_string.push_str("$(");
                    ans_string.push_str(&cmd_string);
                }
            }
            // $だけなのでそのまま残しておく
            else {
                ans_string.push(ch);
            }
        } else {
            ans_string.push(ch);
        }
    }

    Ok(ans_string)
}

#[cfg(test)]
mod environment_tests {
    use super::*;
    use crate::commands::{Command, EnvironmentAction};
    use crate::environment::{expand_variables, set_var};
    use crate::handlers::handle_environment;
    use crate::parser::parse_command;

    // ========================================
    // PR #54: Environment Variable Management Tests
    // ========================================

    #[test]
    fn test_env_command_list_all() {
        // When: env コマンドを引数なしで実行
        let result = handle_environment(EnvironmentAction::List).unwrap();

        // Then: システム環境変数が表示される
        assert!(result.contains("PATH="));
        assert!(!result.is_empty());
    }

    #[test]
    fn test_env_set_and_get() {
        // Given: 新しい環境変数を設定
        handle_environment(EnvironmentAction::Set(
            "TEST_VAR".to_string(),
            "test_value".to_string(),
        ))
        .unwrap();

        // When: その変数を取得
        let result = handle_environment(EnvironmentAction::Show("TEST_VAR".to_string())).unwrap();

        // Then: 設定した値が返される
        assert_eq!(result.trim(), "test_value");
    }

    #[test]
    fn test_env_set_overwrites_existing() {
        // Given: 変数を設定
        handle_environment(EnvironmentAction::Set(
            "OVERWRITE_TEST".to_string(),
            "original".to_string(),
        ))
        .unwrap();

        // When: 同じ変数に別の値を設定
        handle_environment(EnvironmentAction::Set(
            "OVERWRITE_TEST".to_string(),
            "updated".to_string(),
        ))
        .unwrap();

        // Then: 新しい値で上書きされる
        let result =
            handle_environment(EnvironmentAction::Show("OVERWRITE_TEST".to_string())).unwrap();
        assert_eq!(result.trim(), "updated");
    }

    #[test]
    fn test_env_show_nonexistent_variable() {
        // When: 存在しない変数を表示しようとする
        let result = handle_environment(EnvironmentAction::Show("NONEXISTENT_VAR".to_string()));

        // Then: エラーが返される
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Environment variable 'NONEXISTENT_VAR' not found")
        );
    }

    #[test]
    fn test_parse_env_command_list() {
        // When: env コマンドをパース
        let cmd = parse_command("env").unwrap();

        // Then: Environment::List コマンドが生成される
        assert!(matches!(
            cmd,
            Command::Environment {
                action: EnvironmentAction::List
            }
        ));
    }

    #[test]
    fn test_parse_env_command_set() {
        // When: env VAR=value コマンドをパース
        let cmd = parse_command("env TEST_VAR=test_value").unwrap();

        // Then: Environment::Set コマンドが生成される
        assert!(matches!(cmd, Command::Environment { 
            action: EnvironmentAction::Set(var, val) 
        } if var == "TEST_VAR" && val == "test_value"));
    }

    #[test]
    fn test_parse_env_command_show() {
        // When: env VAR コマンドをパース
        let cmd = parse_command("env PATH").unwrap();

        // Then: Environment::Show コマンドが生成される
        assert!(matches!(cmd, Command::Environment { 
            action: EnvironmentAction::Show(var) 
        } if var == "PATH"));
    }

    #[test]
    fn test_basic_brace_variable_expansion() {
        // Given: 環境変数を設定
        set_var("PREFIX", "test");
        set_var("PATH", "/usr/bin");

        // When/Then: 基本的な ${VAR} 展開
        assert_eq!(expand_variables("${PREFIX}"), "test");
        assert_eq!(expand_variables("${PATH}"), "/usr/bin");
    }

    #[test]
    fn test_mixed_expansion_styles() {
        // Given: 環境変数を設定
        set_var("USER", "alice");
        set_var("HOST", "server");

        // When/Then: $VAR と ${VAR} の混在
        assert_eq!(expand_variables("$USER@${HOST}.com"), "alice@server.com");
        assert_eq!(expand_variables("${USER} on $HOST"), "alice on server");
    }

    #[test]
    fn test_multiple_variable_expansion() {
        // Given: 複数の環境変数
        set_var("FIRST", "Hello");
        set_var("SECOND", "World");
        set_var("THIRD", "!");

        // When/Then: 複数変数を展開
        assert_eq!(expand_variables("$FIRST $SECOND$THIRD"), "Hello World!");
        assert_eq!(
            expand_variables("${FIRST} ${SECOND} ${THIRD}"),
            "Hello World !"
        );
    }

    #[test]
    fn test_nonexistent_variable_expansion() {
        // When/Then: 存在しない変数（空文字列に展開）
        assert_eq!(expand_variables("$NONEXISTENT"), "");
        assert_eq!(expand_variables("${MISSING}"), "");
    }

    #[test]
    fn test_brace_error_cases() {
        // When/Then: ブレース記法のエラーケース
        assert_eq!(expand_variables("${VAR"), "${VAR"); // 閉じブレースなし
        assert_eq!(expand_variables("${}"), "${}"); // 空の変数名
        assert_eq!(expand_variables("${INCOMPLETE text"), "${INCOMPLETE text"); // 途中終了
    }

    #[test]
    fn test_dollar_edge_cases() {
        // When/Then: $ のエッジケース
        assert_eq!(expand_variables("Price $100"), "Price "); // 数字始まり変数は無効→空文字列
        assert_eq!(expand_variables("$$"), "$$"); // 連続$
        assert_eq!(expand_variables("$"), "$"); // 単独$
    }

    #[test]
    fn test_system_variable_expansion() {
        // When/Then: システム環境変数の展開
        let result = expand_variables("Home: $HOME");
        assert!(result.starts_with("Home: /") || result == "Home: "); // HOMEがない環境もある
        assert!(!result.contains("$HOME"));

        let path_result = expand_variables("Path: $PATH");
        assert!(path_result.contains("Path: "));
        assert!(!path_result.contains("$PATH"));
    }

    #[test]
    fn test_cat_command_with_variable_filename() {
        // Given: 環境変数を設定
        set_var("FILENAME", "test.txt");

        let cmd = parse_command("cat $FILENAME").unwrap();

        // パース時点では変数展開されない
        assert!(matches!(cmd.clone(), Command::Cat { filename } if filename == "$FILENAME"));

        // expand_variablesメソッドで展開
        let expanded_cmd = cmd.expand_variables();
        assert!(matches!(expanded_cmd, Command::Cat { filename } if filename == "test.txt"));
    }

    #[test]
    fn test_write_command_with_variable_expansion() {
        // Given: ファイル名と内容を環境変数に設定
        set_var("OUTPUT", "output.txt");
        set_var("MESSAGE", "Hello File");

        // When: writeコマンドで変数展開
        let cmd = parse_command("write $OUTPUT $MESSAGE").unwrap();

        // パース時点では変数展開されない
        match &cmd {
            Command::Write { filename, content } => {
                assert_eq!(filename, "$OUTPUT");
                assert_eq!(content, "$MESSAGE");
            }
            _ => panic!("Expected Write command"),
        }

        // expand_variablesメソッドで展開
        let expanded_cmd = cmd.expand_variables();
        assert!(matches!(expanded_cmd, Command::Write { filename, content } 
            if filename == "output.txt" && content == "Hello File"));
    }

    #[test]
    fn test_variable_expansion_in_pipeline() {
        // Given: 環境変数を設定
        set_var("PATTERN", "error");
        set_var("LOGFILE", "app.log");

        // When: パイプラインで変数展開
        let cmd = parse_command("cat $LOGFILE | grep $PATTERN").unwrap();

        // Then: パイプライン内の変数はまだ展開されていない（文字列のまま）
        if let Command::Pipeline { commands } = cmd {
            assert_eq!(commands.len(), 2);
            // パイプラインのコマンドは文字列として保持
            assert!(commands[0].contains("$LOGFILE"));
            assert!(commands[1].contains("$PATTERN"));
        } else {
            panic!("Expected pipeline command");
        }
    }

    #[test]
    fn test_variable_expansion_in_redirect() {
        // Given: 環境変数を設定
        set_var("INPUT", "source.txt");
        set_var("OUTPUT", "dest.txt");

        // When: リダイレクトで変数展開
        let cmd = parse_command("cat $INPUT > $OUTPUT").unwrap();

        // Then: パース時点では展開されていない
        if let Command::Redirect {
            command,
            redirect_type,
            target,
        } = cmd
        {
            assert_eq!(redirect_type, ">");
            assert_eq!(target, "$OUTPUT");

            match *command {
                Command::Cat { filename } => {
                    assert_eq!(filename, "$INPUT");
                }
                _ => panic!("Expected Cat command"),
            }
        } else {
            panic!("Expected redirect command");
        }
    }

    #[test]
    fn test_variable_expansion_with_background() {
        // Given: 環境変数を設定
        set_var("CMD", "echo");
        set_var("MSG", "hello");

        // When: バックグラウンド実行で変数展開
        let result = parse_command("$CMD $MSG &");

        // パース時点では$CMDがコマンド名として認識されずエラーになる可能性
        // これは現在の実装の制限
        assert!(result.is_err() || matches!(result, Ok(Command::Background { .. })));
    }

    #[test]
    fn test_variable_expansion_with_special_characters() {
        // Given: 特殊文字を含む値
        set_var("EMAIL", "user@domain.com");
        set_var("PATH_WITH_SPACES", "/path with spaces");

        // When: 特殊文字を含む変数を展開
        assert_eq!(
            expand_variables("Contact: $EMAIL"),
            "Contact: user@domain.com"
        );
        assert_eq!(
            expand_variables("Dir: ${PATH_WITH_SPACES}"),
            "Dir: /path with spaces"
        );
    }

    #[test]
    fn test_empty_variable_expansion() {
        // Given: 空文字列の環境変数
        set_var("EMPTY", "");

        // When: 空変数を展開
        assert_eq!(expand_variables("Value: $EMPTY end"), "Value:  end");
        assert_eq!(expand_variables("${EMPTY}test"), "test");
    }

    #[test]
    fn test_variable_expansion_preserves_quotes() {
        // Given: 環境変数を設定
        set_var("QUOTED", "with spaces");

        // When: クォート内で変数展開
        assert_eq!(
            expand_variables("\"Message: $QUOTED\""),
            "\"Message: with spaces\""
        );
        assert_eq!(expand_variables("'${QUOTED}'"), "'with spaces'");
    }

    #[test]
    fn test_alphanumeric_variable_names() {
        // Given: 数字を含む変数名
        set_var("VAR1", "first");
        set_var("VAR2", "second");
        set_var("PATH123", "custom_path");

        // When/Then: 数字を含む変数名の展開
        assert_eq!(expand_variables("$VAR1 and $VAR2"), "first and second");
        assert_eq!(expand_variables("${PATH123}"), "custom_path");
    }

    #[test]
    fn test_underscore_in_variable_names() {
        // Given: アンダースコアを含む変数名
        set_var("MY_VAR", "underscore_value");
        set_var("TEST_123", "test_value");

        // When/Then: アンダースコア変数の展開
        assert_eq!(expand_variables("$MY_VAR"), "underscore_value");
        assert_eq!(expand_variables("${TEST_123}"), "test_value");
    }

    #[test]
    fn test_session_vs_system_variable_priority() {
        // Given: システム変数と同名のセッション変数を設定
        set_var("PATH", "/custom/path");

        // When: 変数を展開
        let result = expand_variables("$PATH");

        // Then: セッション変数が優先される
        assert_eq!(result, "/custom/path");
    }

    #[test]
    fn test_complex_expansion_scenario() {
        // Given: 複雑なシナリオの環境変数
        set_var("PROJECT", "myapp");
        set_var("VERSION", "1.0");
        set_var("ENV", "prod");

        // When: 複雑な変数展開
        let result = expand_variables("${PROJECT}-${VERSION}-${ENV}.tar.gz");

        // Then: 正しく展開される
        assert_eq!(result, "myapp-1.0-prod.tar.gz");
    }

    #[test]
    fn test_debug_variable_boundaries() {
        // デバッグ: 変数名の境界確認
        set_var("VAR", "value");
        set_var("_UNDERSCORE", "underscore_value");

        // 数字との境界（bash準拠）
        assert_eq!(expand_variables("$VAR123"), ""); // VAR123という変数（存在しない）
        assert_eq!(expand_variables("${VAR}123"), "value123"); // VAR + "123"

        // 記号との境界
        assert_eq!(expand_variables("$VAR.txt"), "value.txt"); // VAR + ".txt"
        assert_eq!(expand_variables("$VAR/path"), "value/path"); // VAR + "/path"

        // アンダースコア始まり
        assert_eq!(expand_variables("$_UNDERSCORE"), "underscore_value"); // 有効
    }

    // ========================================
    // PR #56: Command Substitution Tests
    // ========================================

    #[test]
    fn test_basic_command_substitution() {
        // Given: pwdコマンドの出力を置換
        let input = "Current dir: $(pwd)";

        // When: コマンド置換を実行
        let result = expand_command_substitution(input).unwrap();

        // Then: pwdの出力が含まれる
        assert!(result.starts_with("Current dir: /"));
        assert!(!result.contains("$(pwd)"));
    }

    #[test]
    fn test_multiple_command_substitutions() {
        // Given: 複数のコマンド置換
        let input = "First: $(echo hello) Second: $(echo world)";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: 両方が置換される
        assert_eq!(result, "First: hello Second: world");
    }

    #[test]
    fn test_nested_command_substitution() {
        // Given: ネストされたコマンド置換
        let input = "Result: $(echo $(echo nested))";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: 正しく評価される
        assert_eq!(result, "Result: nested");
    }

    #[test]
    fn test_command_substitution_with_pipe() {
        // Given: echoコマンドでパイプをシミュレート
        let input = "Result: $(echo hello | cat)";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: パイプラインが実行される
        assert_eq!(result, "Result: hello");
    }

    #[test]
    fn test_failed_command_substitution() {
        // Given: 失敗するコマンド
        let input = "Error: $(nonexistent_command)";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: 空文字列に置換される
        assert_eq!(result, "Error: ");
    }

    #[test]
    fn test_unclosed_command_substitution() {
        // Given: 閉じていない括弧
        let input = "Incomplete: $(echo hello";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: 元の文字列が保持される
        assert_eq!(result, input);
    }

    #[test]
    fn test_empty_command_substitution() {
        // Given: 空のコマンド
        let input = "Empty: $() end";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: 空文字列に置換
        assert_eq!(result, "Empty:  end");
    }

    #[test]
    fn test_command_substitution_preserves_quotes() {
        // Given: クォート内でのコマンド置換
        let input = r#"Message: "$(echo hello world)""#;

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: クォートが保持される
        assert_eq!(result, r#"Message: "hello world""#);
    }

    #[test]
    fn test_command_substitution_with_variables() {
        // Given: 変数を含むコマンド
        set_var("NAME", "test");
        let input = "Result: $(echo $NAME)";

        // When: 変数展開してから置換実行
        let expanded = expand_variables(input);
        let result = expand_command_substitution(&expanded).unwrap();

        // Then: 変数が展開されてから実行される
        assert_eq!(result, "Result: test");
    }

    #[test]
    fn test_complex_nested_substitution() {
        // Given: 複雑なネスト
        let input = "$(echo Result: $(echo $(echo deep)))";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: 正しく評価される
        assert_eq!(result, "Result: deep");
    }

    #[test]
    fn test_command_substitution_trims_newline() {
        // Given: 改行を含む出力
        let input = "Dir: $(pwd)!";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: 改行が除去される
        assert!(result.ends_with("!"));
        assert!(!result.contains('\n'));
    }

    #[test]
    fn test_dollar_without_parenthesis() {
        // Given: $だけの文字列
        let input = "Price is $100";

        // When: 置換実行
        let result = expand_command_substitution(input).unwrap();

        // Then: 変更されない
        assert_eq!(result, "Price is $100");
    }

    #[test]
    fn test_mixed_expansion_and_substitution() {
        // Given: 変数展開とコマンド置換の混在
        set_var("PREFIX", "Hello");
        let input = "$PREFIX $(echo World)!";

        // When: 両方の展開を実行
        let var_expanded = expand_variables(input);
        let result = expand_command_substitution(&var_expanded).unwrap();

        // Then: 両方が正しく処理される
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_expand_variables_method() {
        // Given: 環境変数を設定
        set_var("FILE", "test.txt");
        set_var("MSG", "Hello World");

        // Catコマンドのテスト
        let cat_cmd = Command::Cat {
            filename: "$FILE".to_string(),
        };
        let expanded_cat = cat_cmd.expand_variables();
        assert!(matches!(expanded_cat, Command::Cat { filename } if filename == "test.txt"));

        // Echoコマンドのテスト
        let echo_cmd = Command::Echo {
            message: "$MSG".to_string(),
        };
        let expanded_echo = echo_cmd.expand_variables();
        assert!(matches!(expanded_echo, Command::Echo { message } if message == "Hello World"));

        // 複数変数のテスト
        unsafe {
            std::env::set_var("USER", "testuser");
        }
        let write_cmd = Command::Write {
            filename: "$FILE".to_string(),
            content: "$MSG from $USER".to_string(),
        };
        let expanded_write = write_cmd.expand_variables();
        match expanded_write {
            Command::Write { filename, content } => {
                assert_eq!(filename, "test.txt");
                assert!(content.contains("Hello World from"));
            }
            _ => panic!("Expected Write command"),
        }
        unsafe {
            std::env::remove_var("USER");
        }
    }
}
