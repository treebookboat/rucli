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

    // ループ開始前
    debug!("Entering command loop");

    // 入力された命令の処理を行う
    loop {
        // プロンプトの追加
        print!("> ");
        io::stdout().flush().unwrap();

        // 入力された文字列の読み取り
        let input = read_input();
        debug!("Received input: {input}"); // 入力内容の記録

        // シンプルなディスパッチ
        if parser::contains_heredoc(&input) {
            handle_heredoc_command(&input);
        } else {
            handle_normal_command(&input);
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
