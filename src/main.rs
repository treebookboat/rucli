mod commands;
mod error;
mod handlers;
mod parser;

use commands::execute_command;

use std::io::{self, Write};

use crate::parser::parse_command;

fn main() {
    println!("Hello, rucli!");

    // 入力された命令の処理を行う
    loop {
        // プロンプトの追加
        print!("> ");
        io::stdout().flush().unwrap();

        // 入力された文字列の読み取り
        let input = read_input();

        // コマンドのパース
        match parse_command(&input) {
            // 命令の実行
            Ok(command) => execute_command(command),
            Err(error) => eprintln!("{}", error),
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
