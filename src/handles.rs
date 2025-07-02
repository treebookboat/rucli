use std::{fs, path::Path, process};

use crate::commands::COMMANDS;

// ヘルプ命令の中身
pub fn handle_help() {
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
pub fn handle_repeat(count : i32 , message : &str)
{
    for _ in 0..count{
        println!("{}", message);
    }
}

// path内のテキスト表示
pub fn handle_cat(filename : &str)
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
pub fn handle_write(filename : &str, content : &str){
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
pub fn handle_ls()
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
pub fn handle_exit() {
    println!("good bye");
    // 0が正常終了、1以上がエラー
    process::exit(0);
}