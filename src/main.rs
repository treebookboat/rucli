//! rucliのエントリポイント

mod alias;
mod commands;
mod environment;
mod error;
mod functions;
mod handlers;
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

use crate::parser::parse_command;

/// ブロック入力の状態
#[derive(Debug, Copy, Clone, PartialEq)]
enum BlockState {
    Normal,         // 通常の入力
    ExpectingDo,    // for/while の後、do を待つ
    ExpectingThen,  // if の後、then を待つ
    ExpectingDone,  // do の後、done を待つ
    ExpectingFi,    // then の後、fi を待つ
    ExpectingElse,  // then の後、else/fi を待つ
    ExpectingBrace, // function の後、} を待つ
}

/// ブロック入力を管理する構造体
struct BlockInputCollector {
    lines: Vec<String>,
    state: BlockState,
    depth: i32,
}

impl BlockInputCollector {
    fn new() -> Self {
        BlockInputCollector {
            lines: Vec::new(),
            state: BlockState::Normal,
            depth: 0,
        }
    }

    /// 行を追加し、次の状態を返す
    /// Noneなら入力完了
    fn add_line(&mut self, line: &str) -> Option<BlockState> {
        // 現在の行に新しく追加
        self.lines.push(line.to_string());

        // キーワードを検出
        let keyword = Self::detect_keyword(line);

        // 現在の状態とキーワードに基づいて次の状態を決定する
        match (self.state, keyword) {
            // === for/while ループ ===
            (BlockState::Normal, Some("for")) => {
                self.state = BlockState::ExpectingDo;
                Some(BlockState::ExpectingDo)
            }
            (BlockState::Normal, Some("while")) => {
                self.state = BlockState::ExpectingDo;
                Some(BlockState::ExpectingDo)
            }

            // === if 文 ===
            (BlockState::Normal, Some("if")) => {
                self.state = BlockState::ExpectingThen;
                Some(BlockState::ExpectingThen)
            }
            (BlockState::ExpectingThen, Some("then")) => {
                self.state = BlockState::ExpectingFi;
                Some(BlockState::ExpectingFi)
            }
            (BlockState::ExpectingFi, Some("else")) => {
                self.state = BlockState::ExpectingFi; // 状態維持
                Some(BlockState::ExpectingFi)
            }
            (BlockState::ExpectingFi, Some("fi")) => {
                None // 完了
            }

            // === 関数定義 ===
            (BlockState::Normal, Some("function")) => {
                self.state = BlockState::ExpectingBrace;
                Some(BlockState::ExpectingBrace)
            }
            (BlockState::ExpectingBrace, Some("}")) => {
                None // 完了
            }

            // doの入力後のdone待ち
            (BlockState::ExpectingDo, Some("do")) => {
                self.state = BlockState::ExpectingDone;
                Some(BlockState::ExpectingDone)
            }

            // === ネスト処理（while も追加） ===
            (BlockState::ExpectingDone, Some("for") | Some("while")) => {
                self.depth += 1;
                Some(BlockState::ExpectingDone)
            }
            // すでにネストが一つ深い状態のdoneはdepthを一つ下げる
            (BlockState::ExpectingDone, Some("done")) if self.depth > 0 => {
                self.depth -= 1;
                Some(BlockState::ExpectingDone)
            }
            (BlockState::ExpectingDone, Some("done")) if self.depth == 0 => {
                None // 完了
            }

            _ => Some(self.state),
        }
    }

    /// 行からキーワードを検出
    fn detect_keyword(line: &str) -> Option<&str> {
        let trimmed = line.trim();
        if trimmed.starts_with("for ") {
            return Some("for");
        }
        if trimmed.starts_with("while ") {
            return Some("while");
        }
        if trimmed.starts_with("if ") {
            return Some("if");
        }
        if trimmed.starts_with("function ") {
            // 同一行に { がある場合は単一行として扱う
            if trimmed.contains("{") && trimmed.contains("}") {
                return None; // 通常のコマンド
            }
            // 複数行の関数定義
            return Some("function");
        }
        if trimmed == "do" {
            return Some("do");
        }
        if trimmed == "then" {
            return Some("then");
        }
        if trimmed == "else" {
            return Some("else");
        }
        if trimmed == "done" {
            return Some("done");
        }
        if trimmed == "fi" {
            return Some("fi");
        }
        if trimmed == "}" {
            return Some("}");
        }
        None
    }

    /// 蓄積された入力を一行に統合
    fn get_complete_command(&self) -> String {
        let mut result = String::new();

        for (i, line) in self.lines.iter().enumerate() {
            let trimmed = line.trim();

            // 空行はスキップ
            if trimmed.is_empty() {
                continue;
            }

            // 行を追加
            result.push_str(trimmed);

            // 最後の行でなければ区切り文字を追加
            if i < self.lines.len() - 1 {
                let next_line = self
                    .lines
                    .get(i + 1)
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty());

                if let Some(next) = next_line {
                    // 現在の行と次の行に基づいて区切り文字を決定
                    match (trimmed, next) {
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
                        // キーワード行の前はスペースのみ
                        (_, next) if Self::is_keyword_line(next) => {
                            result.push(' ');
                        }
                        // その他の場合はセミコロン
                        _ => {
                            result.push_str("; ");
                        }
                    }
                }
            }
        }

        result
    }

    /// キーワードのみの行かチェック
    fn is_keyword_line(line: &str) -> bool {
        matches!(line, "do" | "then" | "else" | "done" | "fi" | "{" | "}")
    }

    /// セミコロン不要な行かチェック  
    fn is_continuation(line: &str) -> bool {
        // これらのキーワードで始まる行は次の行に続く
        line.starts_with("for ")
            || line.starts_with("while ")
            || line.starts_with("if ")
            || line.starts_with("function ")
            || line == "then"
            || line == "else"
            || line == "do"
            || line == "{"
    }

    /// 現在のプロンプトを取得
    fn get_prompt(&self) -> &str {
        match self.state {
            BlockState::Normal => "> ",
            _ => ">> ",
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
        if let Some(_next_state) = block_collector.add_line(&input) {
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

    // 内容を行ごとに分割
    let lines = contents.lines();

    // 各行を順番に処理
    for (line_num, line) in lines.enumerate() {
        // 行の前処理
        let line = line.trim();

        // シバンコメント。空行スキップ
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        handle_normal_command(line);
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
    match parse_command(input) {
        Ok(command) => {
            debug!("Command parsed successfully");
            let start = Instant::now();
            if let Err(err) = execute_command(command) {
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
            match commands::execute_command_get_output(command, Some(input)) {
                Ok(output) => {
                    if !output.is_empty() {
                        println!("{}", output);
                    }
                }
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
            line.strip_prefix('\t').unwrap_or(&line)
        } else {
            &line
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

        assert_eq!(
            collector.add_line("for i in 1 2 3"),
            Some(BlockState::ExpectingDo)
        );
        assert_eq!(collector.get_prompt(), ">> ");

        assert_eq!(collector.add_line("do"), Some(BlockState::ExpectingDone));
        assert_eq!(
            collector.add_line("  echo $i"),
            Some(BlockState::ExpectingDone)
        );
        assert_eq!(collector.add_line("done"), None);

        assert_eq!(
            collector.get_complete_command(),
            "for i in 1 2 3; do echo $i; done"
        );
    }

    #[test]
    fn test_while_loop() {
        let mut collector = BlockInputCollector::new();

        collector.add_line("while test -f flag");
        collector.add_line("do");
        collector.add_line("  cat flag");
        collector.add_line("  rm flag");
        collector.add_line("done");

        let cmd = collector.get_complete_command();
        assert!(cmd.contains("while test -f flag"));
        assert!(cmd.contains("do cat flag"));
        assert!(cmd.contains("rm flag"));
        assert!(cmd.contains("done"));
    }

    #[test]
    fn test_if_then_else_fi() {
        let mut collector = BlockInputCollector::new();

        assert_eq!(
            collector.add_line("if pwd"),
            Some(BlockState::ExpectingThen)
        );
        assert_eq!(collector.add_line("then"), Some(BlockState::ExpectingFi));
        assert_eq!(
            collector.add_line("  echo exists"),
            Some(BlockState::ExpectingFi)
        );
        assert_eq!(collector.add_line("else"), Some(BlockState::ExpectingFi));
        assert_eq!(
            collector.add_line("  echo not found"),
            Some(BlockState::ExpectingFi)
        );
        assert_eq!(collector.add_line("fi"), None);

        let cmd = collector.get_complete_command();
        assert_eq!(cmd, "if pwd; then echo exists; else echo not found; fi");
    }

    #[test]
    fn test_nested_for_loops() {
        let mut collector = BlockInputCollector::new();

        collector.add_line("for i in 1 2");
        collector.add_line("do");
        assert_eq!(collector.depth, 0);

        collector.add_line("  for j in a b");
        assert_eq!(collector.depth, 1);

        collector.add_line("  do");
        collector.add_line("    echo $i$j");
        collector.add_line("  done");
        assert_eq!(collector.depth, 0);

        collector.add_line("done");

        let cmd = collector.get_complete_command();
        assert!(cmd.contains("for i in 1 2"));
        assert!(cmd.contains("for j in a b"));
    }

    #[test]
    fn test_function_multiline() {
        let mut collector = BlockInputCollector::new();

        assert_eq!(
            collector.add_line("function test()"),
            Some(BlockState::ExpectingBrace)
        );
        assert_eq!(collector.add_line("{"), Some(BlockState::ExpectingBrace));
        assert_eq!(
            collector.add_line("  echo Hello"),
            Some(BlockState::ExpectingBrace)
        );
        assert_eq!(
            collector.add_line("  echo World"),
            Some(BlockState::ExpectingBrace)
        );
        assert_eq!(collector.add_line("}"), None);

        let cmd = collector.get_complete_command();
        assert!(cmd.contains("function test()"));
        assert!(cmd.contains("echo Hello"));
        assert!(cmd.contains("echo World"));
    }

    #[test]
    fn test_empty_lines_ignored() {
        let mut collector = BlockInputCollector::new();

        collector.add_line("for i in 1 2 3");
        collector.add_line("do");
        collector.add_line(""); // 空行
        collector.add_line("  echo $i");
        collector.add_line(""); // 空行
        collector.add_line("done");

        let cmd = collector.get_complete_command();
        assert_eq!(cmd, "for i in 1 2 3; do echo $i; done");
    }

    #[test]
    fn test_state_transitions() {
        let mut collector = BlockInputCollector::new();

        // Normal → ExpectingDo
        assert_eq!(collector.state, BlockState::Normal);
        collector.add_line("for i in 1 2 3");
        assert_eq!(collector.state, BlockState::ExpectingDo);

        // ExpectingDo → ExpectingDone
        collector.add_line("do");
        assert_eq!(collector.state, BlockState::ExpectingDone);

        // Complete
        collector.add_line("done");
        assert_eq!(collector.state, BlockState::ExpectingDone); // 変更されない
    }

    #[test]
    fn test_detect_keyword() {
        assert_eq!(
            BlockInputCollector::detect_keyword("for i in 1 2 3"),
            Some("for")
        );
        assert_eq!(
            BlockInputCollector::detect_keyword("while true"),
            Some("while")
        );
        assert_eq!(BlockInputCollector::detect_keyword("if test"), Some("if"));
        assert_eq!(BlockInputCollector::detect_keyword("do"), Some("do"));
        assert_eq!(
            BlockInputCollector::detect_keyword("  done  "),
            Some("done")
        );
        assert_eq!(BlockInputCollector::detect_keyword("echo hello"), None);
    }

    #[test]
    fn test_is_keyword_line() {
        assert!(BlockInputCollector::is_keyword_line("do"));
        assert!(BlockInputCollector::is_keyword_line("then"));
        assert!(BlockInputCollector::is_keyword_line("done"));
        assert!(!BlockInputCollector::is_keyword_line("echo"));
        assert!(!BlockInputCollector::is_keyword_line("for i"));
    }

    #[test]
    fn test_is_continuation() {
        assert!(BlockInputCollector::is_continuation("for i in 1 2 3"));
        assert!(BlockInputCollector::is_continuation("while test"));
        assert!(BlockInputCollector::is_continuation("if pwd"));
        assert!(!BlockInputCollector::is_continuation("echo hello"));
        assert!(!BlockInputCollector::is_continuation("done"));
    }
}
