use std::{io::{self, Write},process,fs,path::Path};

// 実行できるコマンド群
enum Command {
    Help,
    Echo{message : String},
    Repeat{count : i32, message : String},
    Cat{filename : String},
    Write{filename : String, content : String},
    Ls,
    Exit,
}

struct CommandInfo {
    name: &'static str,
    description: &'static str,
    usage: &'static str,
    min_args: usize,
    max_args: Option<usize>,
}

const COMMANDS: &[CommandInfo] = &[
    CommandInfo {
        name: "help",
        description: "Show this help message",
        usage: "help",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "echo",
        description: "Display message",
        usage: "echo <message...>",
        min_args: 1,
        max_args: None,
    },
    CommandInfo {
        name: "cat",
        description: "Display file contents",
        usage: "cat <filename>",
        min_args: 1,
        max_args: Some(1),
    },
    CommandInfo {
        name: "write",
        description: "Write content to file",
        usage: "write <filename> <content...>",
        min_args: 2,
        max_args: None,
    },
    CommandInfo {
        name: "ls",
        description: "List directory contents",
        usage: "ls",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "repeat",
        description: "Repeat message count times",
        usage: "repeat <count> <message...>",
        min_args: 2,
        max_args: None,
    },
    CommandInfo {
        name: "exit",
        description: "Exit the program",
        usage: "exit",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "quit",
        description: "Exit the program",
        usage: "quit",
        min_args: 0,
        max_args: Some(0),
    },
];

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
        ["echo", message @ ..] => Ok(Command::Echo{message : message.join(" ")}),
        ["cat" , filename] => Ok(Command::Cat { filename : filename.to_string() }),
        ["cat"] => Err("Error: cat requires a filename".to_string()),
        ["write"] => Err("Error : write requires a filename and a content".to_string()),
        ["write", _] => Err("Error : write requires a content".to_string()),
        ["write", filename,content @ ..] => Ok(Command::Write { filename : filename.to_string(), content : content.join(" ")}),
        ["ls"] => Ok(Command::Ls),
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
        Command::Cat { filename } => handle_cat(&filename),
        Command::Echo{message} => println!("{}", message),
        Command::Write { filename, content } => handle_write(&filename, &content),
        Command::Repeat{count, message} => handle_repeat(count, &message),
        Command::Ls => handle_ls(),
        Command::Exit => handle_exit(),
    }
}

// ヘルプ命令の中身
fn handle_help() {
    println!("Available commands:");

    // 左寄せでそろえるために最長のusageを計算
    let max_width = COMMANDS
    .iter()
    .map(|cmd| cmd.usage.len())
    .max()
    .unwrap_or(0);

    for cmd in COMMANDS {
        // cmd.usage と cmd.description を表示
        println!("  {:<width$} - {}", cmd.usage, cmd.description, width = max_width);
    }
}

// 文字列をcount回表示
fn handle_repeat(count : i32 , message : &str)
{
    for _ in 0..count{
        println!("{}", message);
    }
}

// path内のテキスト表示
fn handle_cat(filename : &str)
{
    if Path::new(filename).is_dir() {
        eprintln!("Error: '{}' is a directory", filename);
        return;
    }

    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("{}",contents)
        }
        Err(error) =>{
            eprintln!("Error: Failed to cat file '{}': {}", filename, error);
        }
    }
}

// pathのファイルにテキスト追加
fn handle_write(filename : &str, content : &str){
    match fs::write(filename, content)
    {
        Ok(_) => {
            println!("File written successfully: {}", filename);
        }
        Err(error) => {
            eprintln!("Error: Failed to write file '{}': {}", filename, error);
        }
    }
}

// 現在のディレクトリ内のファイル/ディレクトリを表示
fn handle_ls()
{
    match fs::read_dir(".")
    {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        let path = e.path();
                        let file_name = e.file_name();
                        let name = file_name.to_str().unwrap_or("???");
                        if path.is_dir() {
                            println!("{}/", name);
                        } else {
                            println!("{}", name);
                        }
                    }
                    Err(error) => {
                        eprintln!("Error reading entry: {}", error);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Error: Failed to read directory: {}", error);
        }
    }
}

// プログラムを終了する
fn handle_exit() {
    println!("good bye");
    // 0が正常終了、1以上がエラー
    process::exit(0);
}