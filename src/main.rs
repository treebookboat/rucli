use std::{io::{self, Write},process};

// 実行できるコマンド群
enum Command {
    Help,
    Echo(String),
    Repeat{count : i32, message : String},
    Exit,
}

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
            Err(error) => println!("{}", error)
        }
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

// 文字列のパース
fn parse_command(input : &str) -> Result<Command, String>{
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.as_slice() {
        ["help"] => Ok(Command::Help),
        ["echo"] => Err("Error : echo requires a message".to_string()),
        ["echo", message @ ..] => Ok(Command::Echo(message.join(" "))),
        ["repeat", count , message @ ..] => {
            match count.parse::<i32>() {
                Ok(count) if count > 0 => Ok(Command::Repeat{count, message : message.join(" ") }),
                Ok(_) => Err("Error : count must be positive".to_string()),
                Err(_) => Err(format!("Error: {} isn't a valid number", count)),
            }
        },
        ["exit"] | ["quit"] => Ok(Command::Exit),
        commands => Err(format!("Unknown command: {}", commands.join(" "))),
    }
}

// 命令の実行
fn execute_command(command : Command)
{
    match command {
        Command::Help => handle_help(),
        Command::Echo(message) => println!("{}", message),
        Command::Repeat{count, message} => handle_repeat(count, &message),
        Command::Exit => handle_exit(),
    }
}

// ヘルプ命令の中身
fn handle_help() {
    println!("help - show help message");
    println!("echo - display message");
    println!("repeat <count> <message> - repeat message count times");
    println!("exit - exit the program");
    println!("quit - exit the program");    
}

// 文字列をcount回表示
fn handle_repeat(count : i32 , message : &str)
{
    for _ in 0..count{
        println!("{}", message);
    }
}

// プログラムを終了する
fn handle_exit() {
    println!("good bye");
    // 0が正常終了、1以上がエラー
    process::exit(0);
}