//! rucliのエントリポイント

mod commands;
mod error;
mod handlers;
mod parser;

use commands::execute_command;
use log::{debug, error, info};

use env_logger::Builder;
use log::LevelFilter;
use std::env;
use std::io::{self, Write};
use std::time::Instant;

use crate::parser::parse_command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // コマンドライン引数をチェック
    let debug_mode = env::args().any(|arg| arg == "--debug");

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

        // コマンドのパース
        match parse_command(&input) {
            // 命令の実行
            Ok(command) => {
                debug!("Command parsed successfully"); // パース成功
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
