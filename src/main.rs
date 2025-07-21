//! rucliのエントリポイント

mod alias;
mod commands;
mod environment;
mod error;
mod functions;
mod handlers;
mod history;
mod job;
mod parser;
mod pipeline;
mod redirect;

use commands::execute_command;
use log::{debug, error, info};

use env_logger::Builder;
use log::LevelFilter;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;
use std::{env, fs};

use crate::history::add_history;
use crate::parser::parse_command;

/// ブロック入力を管理する構造体
struct BlockInputCollector {
    lines: Vec<String>,
    depth: i32,
    pending_keywords: Vec<(String, i32)>,
}

impl BlockInputCollector {
    fn new() -> Self {
        BlockInputCollector {
            lines: Vec::new(),
            depth: 0,
            pending_keywords: Vec::new(),
        }
    }

    /// 行を追加し、次の状態を返す
    /// Noneなら入力完了
    fn add_line(&mut self, line: &str) -> bool {
        // 現在の行に新しく追加
        self.lines.push(line.to_string());

        // 新規追加：キーワードを抽出して処理
        let keywords = Self::extract_keywords(line);
        for keyword in keywords {
            match keyword.as_str() {
                "while" | "for" => {
                    self.depth += 1;
                    self.pending_keywords.push(("do".to_string(), self.depth));
                }
                "if" => {
                    self.depth += 1;
                    self.pending_keywords.push(("then".to_string(), self.depth));
                }
                "function" => {
                    self.depth += 1;
                    self.pending_keywords.push(("{".to_string(), self.depth));
                }
                "do" => {
                    self.pending_keywords
                        .retain(|(k, d)| !(k == "do" && *d == self.depth));
                    self.pending_keywords.push(("done".to_string(), self.depth));
                }
                "then" => {
                    self.pending_keywords
                        .retain(|(k, d)| !(k == "then" && *d == self.depth));
                    self.pending_keywords.push(("fi".to_string(), self.depth));
                }
                "{" => {
                    self.pending_keywords
                        .retain(|(k, d)| !(k == "{" && *d == self.depth));
                    self.pending_keywords.push(("}".to_string(), self.depth));
                }
                "done" | "fi" | "}" => {
                    self.pending_keywords
                        .retain(|(k, d)| !(k == keyword.as_str() && *d == self.depth));
                    self.depth -= 1;
                }
                "else" => {
                    // elseは深さを変えない（fiを待ち続ける）
                }
                _ => {}
            }
        }

        // pending_keywordsが空 = 完了
        !self.pending_keywords.is_empty() || self.depth > 0
    }

    fn extract_keywords(line: &str) -> Vec<String> {
        let mut keywords = Vec::new();
        let words: Vec<&str> = line.split_whitespace().collect();

        for word in words.iter() {
            match *word {
                "while" | "for" | "if" | "do" | "then" | "done" | "fi" | "else" | "function"
                | "{" | "}" => {
                    keywords.push(word.to_string());
                }
                _ => {}
            }
        }

        keywords
    }

    /// 蓄積された入力を一行に統合
    fn get_complete_command(&self) -> String {
        let mut result = String::new();

        // 空行を除外したリストを作成
        let non_empty_lines: Vec<&str> = self
            .lines
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for (i, line) in non_empty_lines.iter().enumerate() {
            // 行を追加
            result.push_str(line);

            // 最後の行でなければ区切り文字を追加
            if i < non_empty_lines.len() - 1 {
                let next = non_empty_lines[i + 1];

                match (*line, next) {
                    // "for/while/if ..." の後で "do/then" の前にはセミコロン
                    (curr, "do") if curr.starts_with("for ") || curr.starts_with("while ") => {
                        result.push_str("; ");
                    }
                    (curr, "then") if curr.starts_with("if ") => {
                        result.push_str("; ");
                    }
                    // "do/then/else" の後はスペースのみ
                    ("do" | "then" | "else", _) => {
                        result.push(' ');
                    }
                    // その他の場合はセミコロン
                    _ => {
                        result.push_str("; ");
                    }
                }
            }
        }

        result
    }

    /// 現在のプロンプトを取得
    fn get_prompt(&self) -> &str {
        if self.pending_keywords.is_empty() && self.depth == 0 {
            "> "
        } else {
            ">> "
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 引数を取得
    let args: Vec<String> = env::args().collect();

    // コマンドライン引数をチェック
    let debug_mode = args.iter().any(|arg| arg == "--debug");

    // スクリプトファイルのチェック
    // 引数の最初にスクリプトファイルが入っているかチェック
    let script_file = if args.len() > 1 && !args[1].starts_with("--") {
        Some(&args[1])
    } else {
        None
    };

    // env_loggerの設定
    let mut builder = Builder::from_default_env();

    if debug_mode {
        // RUST_LOG環境変数が設定されている場合はそちら優先
        if env::var("RUST_LOG").is_err() {
            builder.filter_level(LevelFilter::Debug);
        }
    }

    // builderを初期化
    builder.init();

    if debug_mode {
        info!("Debug mode enabled");
    }

    // 実行モードの分岐
    if let Some(filename) = script_file {
        run_script_file(filename)?;
    } else {
        run_interactive_mode()?;
    }

    Ok(())
}

// 対話モードでの実行関数
fn run_interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    // 起動時の作業ディレクトリを記録（デバッグ用）
    let initial_dir = env::current_dir()?;
    debug!("Initial working directory: {initial_dir:?}");

    info!("Starting rucli...");
    println!("Hello, rucli!");

    // BlockInputCollector を追加
    let mut block_collector = BlockInputCollector::new();

    loop {
        // プロンプトを動的に変更
        print!("{}", block_collector.get_prompt());
        io::stdout().flush().unwrap();

        let input = read_input();
        debug!("Received input: {input}");

        // ブロック入力の処理
        if block_collector.add_line(&input) {
            // まだ入力継続中
            continue;
        }

        // 入力完了 - コマンドを取得
        let complete_input = block_collector.get_complete_command();
        block_collector = BlockInputCollector::new(); // リセット

        // 空入力なら次へ
        if complete_input.trim().is_empty() {
            continue;
        }

        // 既存の処理（ヒアドキュメントチェックなど）
        if parser::contains_heredoc(&complete_input) {
            handle_heredoc_command(&complete_input);
        } else {
            handle_normal_command(&complete_input);
        }
    }
}

fn run_script_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ファイルの存在確認
    if !Path::new(filename).exists() {
        eprintln!("Error: Script file {filename} not found");
        std::process::exit(1);
    };

    // ファイル全体を読み込む
    let contents = fs::read_to_string(filename)?;

    let mut block_collector = BlockInputCollector::new();

    for line in contents.lines() {
        let line = line.trim();

        // シバンコメント、空行スキップ
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // すべての行をBlockInputCollectorに渡す（インタラクティブと同じ）
        if !block_collector.add_line(line) {
            // 完了したら実行
            let complete_input = block_collector.get_complete_command();

            if !complete_input.trim().is_empty() {
                if parser::contains_heredoc(&complete_input) {
                    handle_heredoc_command(&complete_input);
                } else {
                    handle_normal_command(&complete_input);
                }
            }

            // リセット
            block_collector = BlockInputCollector::new();
        }
    }

    // ファイル終端で未完了のブロックがある場合
    if block_collector.depth > 0 || !block_collector.pending_keywords.is_empty() {
        eprintln!("Error: Incomplete block structure at end of file");
        std::process::exit(1);
    }

    Ok(())
}

// 入力された文字列の読み取り
fn read_input() -> String {
    let mut input = String::new();

    // 文字列読み取り
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    // 改行文字をトリミングしてString型にしてから返す
    input.trim().to_string()
}

/// ヒアドキュメント付きコマンドを処理
fn handle_heredoc_command(input: &str) {
    if let Some((cmd_str, delimiter, strip_indent)) = parser::parse_heredoc_header(input) {
        debug!(
            "Heredoc header: cmd='{cmd_str}', delimiter='{delimiter}', strip_indent={strip_indent}"
        );

        // 内容を収集
        let content = read_heredoc_content(&delimiter, strip_indent);
        debug!(
            "Collected heredoc content: {} lines",
            content.lines().count()
        );

        // 展開を適用
        let expanded_content = environment::expand_variables(&content);
        let final_content = match environment::expand_command_substitution(&expanded_content) {
            Ok(substituted) => substituted,
            Err(_) => expanded_content,
        };

        // コマンドを実行
        execute_with_input(&cmd_str, &final_content);
    }
}

/// 通常のコマンドを処理（既存のコードを移動）
fn handle_normal_command(input: &str) {
    add_history(input.to_string());

    match parse_command(input) {
        Ok(command) => {
            debug!("Command parsed successfully");
            let start = Instant::now();
            if let Err(err) = execute_command(command, None) {
                error!("Command execution failed: {err}");
                eprintln!("{err}");
            }
            let duration = start.elapsed().as_secs_f64() * 1000.0;
            debug!("処理時間: {duration:?}ms");
        }
        Err(error) => {
            debug!("Parse error occurred: {error}");
            eprintln!("{error}");
        }
    }
}

/// 入力付きでコマンドを実行
fn execute_with_input(cmd_str: &str, input: &str) {
    match parse_command(cmd_str) {
        Ok(command) => {
            let start = Instant::now();
            match commands::execute_command(command, Some(input)) {
                Ok(_) => {}
                Err(err) => {
                    error!("Command execution failed: {err}");
                    eprintln!("{err}");
                }
            }
            let duration = start.elapsed().as_secs_f64() * 1000.0;
            debug!("処理時間: {duration:?}ms");
        }
        Err(error) => {
            debug!("Parse error occurred: {error}");
            eprintln!("{error}");
        }
    }
}

/// ヒアドキュメントの内容を読み取る
fn read_heredoc_content(delimiter: &str, strip_indent: bool) -> String {
    // 空のVec<String>を作成
    let mut lines = Vec::new();
    loop {
        // heredocプロンプト表示
        print!("heredoc> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        // 一行読み取り
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        // デリミタと完全に一致したらbreak
        let line = line.trim_end_matches('\n').trim_end_matches('\r');
        if delimiter == line {
            break;
        }

        // strip_indentがtrueなら先頭タブを削除
        let processed_line = if strip_indent {
            line.strip_prefix('\t').unwrap_or(line)
        } else {
            line
        };

        // Vecに追加
        lines.push(processed_line.to_string());
    }
    lines.join("\n")
}

#[cfg(test)]
mod block_input_tests {
    use super::*;

    #[test]
    fn test_simple_for_loop() {
        let mut collector = BlockInputCollector::new();

        assert!(collector.add_line("for i in 1 2 3")); // 継続
        assert_eq!(collector.get_prompt(), ">> ");

        assert!(collector.add_line("do")); // 継続
        assert!(collector.add_line("  echo $i")); // 継続
        assert!(!collector.add_line("done")); // 完了

        assert_eq!(
            collector.get_complete_command(),
            "for i in 1 2 3; do echo $i; done"
        );
    }

    #[test]
    fn test_while_loop() {
        let mut collector = BlockInputCollector::new();

        assert!(collector.add_line("while test -f flag"));
        assert!(collector.add_line("do"));
        assert!(collector.add_line("  cat flag"));
        assert!(collector.add_line("  rm flag"));
        assert!(!collector.add_line("done"));

        let cmd = collector.get_complete_command();
        assert!(cmd.contains("while test -f flag"));
        assert!(cmd.contains("do cat flag"));
        assert!(cmd.contains("rm flag"));
        assert!(cmd.contains("done"));
    }

    #[test]
    fn test_if_then_else_fi() {
        let mut collector = BlockInputCollector::new();

        assert!(collector.add_line("if pwd")); // 継続
        assert!(collector.add_line("then")); // 継続
        assert!(collector.add_line("  echo exists")); // 継続
        assert!(collector.add_line("else")); // 継続
        assert!(collector.add_line("  echo not found")); // 継続
        assert!(!collector.add_line("fi")); // 完了

        let cmd = collector.get_complete_command();
        assert_eq!(cmd, "if pwd; then echo exists; else echo not found; fi");
    }

    #[test]
    fn test_nested_for_loops() {
        let mut collector = BlockInputCollector::new();

        assert!(collector.add_line("for i in 1 2"));
        assert!(collector.add_line("do"));
        assert_eq!(collector.depth, 1);
        assert_eq!(collector.pending_keywords, vec![("done".to_string(), 1)]);

        assert!(collector.add_line("  for j in a b"));
        assert_eq!(collector.depth, 2);
        assert_eq!(
            collector.pending_keywords,
            vec![("done".to_string(), 1), ("do".to_string(), 2)]
        );

        assert!(collector.add_line("  do"));
        assert_eq!(
            collector.pending_keywords,
            vec![("done".to_string(), 1), ("done".to_string(), 2)]
        );

        assert!(collector.add_line("    echo $i$j"));
        assert!(collector.add_line("  done"));
        assert_eq!(collector.depth, 1);
        assert_eq!(collector.pending_keywords, vec![("done".to_string(), 1)]);

        assert!(!collector.add_line("done")); // 完了
        assert_eq!(collector.depth, 0);
        assert!(collector.pending_keywords.is_empty());

        let cmd = collector.get_complete_command();
        assert!(cmd.contains("for i in 1 2"));
        assert!(cmd.contains("for j in a b"));
    }

    #[test]
    fn test_function_multiline() {
        let mut collector = BlockInputCollector::new();

        assert!(collector.add_line("function test()")); // 継続
        assert!(collector.add_line("{")); // 継続
        assert!(collector.add_line("  echo Hello")); // 継続
        assert!(collector.add_line("  echo World")); // 継続
        assert!(!collector.add_line("}")); // 完了

        let cmd = collector.get_complete_command();
        assert!(cmd.contains("function test()"));
        assert!(cmd.contains("echo Hello"));
        assert!(cmd.contains("echo World"));
    }

    #[test]
    fn test_empty_lines_ignored() {
        let mut collector = BlockInputCollector::new();

        assert!(collector.add_line("for i in 1 2 3"));
        assert!(collector.add_line("do"));
        assert!(collector.add_line("")); // 空行
        assert!(collector.add_line("  echo $i"));
        assert!(collector.add_line("")); // 空行
        assert!(!collector.add_line("done"));

        let cmd = collector.get_complete_command();
        assert_eq!(cmd, "for i in 1 2 3; do echo $i; done");
    }
}
