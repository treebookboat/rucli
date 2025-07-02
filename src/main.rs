use std::{io,process};

fn main() {
    println!("Hello, rucli!");

    // 入力された命令の処理を行う
    loop {
        // 入力された文字列の読み取り
        let user_command = read_input();

        // 命令の実行
        execute_command(&user_command);
    }
}

// 入力された文字列の読み取り
fn read_input() -> String {
    let mut input = String::new();

    // 文字列読み取り
    io::stdin().read_line(&mut input).expect("failed to read line");

    // 改行文字をトリミングしてString型にしてから返す
    input.trim().to_string()
}

// 命令の実行
fn execute_command(command : &str)
{
    match command {
        "help" => show_help(),
        "exit" | "quit" => exit_code(),
        command => println!("Unknown command: {}", command),
    }
}

// ヘルプ命令の中身
fn show_help() {
    println!("help - show help message");
    println!("exit - exit the program");
    println!("quit - exit the program");    
}

// プログラムを終了する
fn exit_code() {
    println!("good bye");
    // 0が正常終了、1以上がエラー
    process::exit(0);
}