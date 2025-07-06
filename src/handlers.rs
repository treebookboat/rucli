use crate::error::{Result, RucliError};
use log::{debug, info, warn};
use std::{fs, io, path::Path, process};

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
        println!(
            "  {:<width$} - {}",
            cmd.usage,
            cmd.description,
            width = max_width
        );
    }
}

// 文字列をcount回表示
pub fn handle_repeat(count: i32, message: &str) {
    for _ in 0..count {
        println!("{}", message);
    }
}

// path内のテキスト表示
pub fn handle_cat(filename: &str) -> Result<()> {
    debug!("Attempting to read file: {}", filename);

    if Path::new(filename).is_dir() {
        warn!("Attempted to cat a directory: {}", filename);

        return Err(RucliError::IoError(io::Error::new(
            io::ErrorKind::Other,
            format!("'{}' is a directory", filename),
        )));
    }

    let contents = fs::read_to_string(filename)?;
    println!("{}", contents);

    // ファイル読み込み成功時
    info!("Successfully read file: {}", filename);

    Ok(())
}

// pathのファイルにテキスト追加
pub fn handle_write(filename: &str, content: &str) -> Result<()> {
    debug!("Writing to file: {} ({} bytes)", filename, content.len());

    fs::write(filename, content)?;
    println!("File written successfully: {}", filename);
    Ok(())
}

// 現在のディレクトリ内のファイル/ディレクトリを表示
pub fn handle_ls() -> Result<()> {
    debug!("Listing current directory contents");

    let entries = fs::read_dir(".")?;
    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_str().unwrap_or("???");
        if path.is_dir() {
            println!("{}/", name);
        } else {
            println!("{}", name);
        }
    }

    Ok(())
}

// プログラムを終了する
pub fn handle_exit() {
    info!("Exiting rucli");
    println!("good bye");
    // 0が正常終了、1以上がエラー
    process::exit(0);
}
