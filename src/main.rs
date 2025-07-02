use std::{io,process};

fn main() {
    println!("Hello, rucli!");

    // 入力された命令の処理を行う
    loop {
        // 入力された文字列の読み取り
        let user_command = read_input();

        // コマンドのパース
        let parse_commands = parse_command(&user_command);

        // 命令の実行
        execute_command(&parse_commands);
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
fn parse_command(input: &str) -> Vec<&str>
{
    input.split_whitespace().collect()
}

// 命令の実行
fn execute_command(commands : &[&str])
{
    match commands {
        ["help"] => handle_help(),
        ["echo"] => println!("Error : echo requires a message"),
        ["echo", message @ ..] => handle_echo(&message.join(" ")),
        ["repeat", count , message @ ..] => {
            match count.parse::<i32>() {
                Ok(count) if count > 0 => handle_repeat(count, &message.join(" ")),
                Ok(_) => println!("Error : count must be positive"),
                Err(_) => println!("Error: {} isn't a valid number", count),
            }
        },
        ["exit"] | ["quit"] => handle_exit(),
        commands => println!("Unknown command: {}", commands.join(" ")),
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

// 文字列の表示
fn handle_echo(message : &str)
{
    println!("{}", message);
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