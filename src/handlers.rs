//! 各コマンドの実装を提供するモジュール

use crate::error::{Result, RucliError};
use log::{debug, info, warn};
use std::{env, fs, io, os::unix::fs::PermissionsExt, path::Path, process};

use crate::commands::COMMANDS;

/// ファイルパーミッションのマスク値
const PERMISSION_MASK: u32 = 0o777;

/// ファイルメタデータをデバッグログに出力する
fn debug_file_metadata(metadata: &fs::Metadata) {
    debug!(
        "File metadata: size={} bytes, permissions={}",
        metadata.len(),
        metadata.permissions().mode() & PERMISSION_MASK,
    );
}

/// ヘルプメッセージを表示する
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

    println!("\nOptions:");
    println!("  --debug    Enable debug mode with detailed logging");
}

/// 文字列をcount回表示
pub fn handle_repeat(count: i32, message: &str) {
    for _ in 0..count {
        println!("{message}");
    }
}

/// ファイルの内容を表示する
///
/// # Errors
///
/// - ファイルが存在しない場合
/// - ディレクトリを指定した場合
/// - 読み取り権限がない場合
pub fn handle_cat(filename: &str) -> Result<()> {
    debug!("Attempting to read file: {filename}");

    if Path::new(filename).is_dir() {
        warn!("Attempted to cat a directory: {filename}");

        return Err(RucliError::IoError(io::Error::other(format!(
            "'{filename}' is a directory"
        ))));
    }

    // ファイル情報表示
    if log::log_enabled!(log::Level::Debug) {
        let metadata = fs::metadata(filename)?;
        debug_file_metadata(&metadata);
    }

    let contents = fs::read_to_string(filename)?;
    println!("{contents}");

    // ファイル読み込み成功時
    info!("Successfully read file: {filename}");

    Ok(())
}

/// ファイルに内容を書き込む
///
/// # Errors
///
/// - 書き込み権限がない場合
/// - ディスク容量不足の場合
pub fn handle_write(filename: &str, content: &str) -> Result<()> {
    debug!("Writing to file: {} ({} bytes)", filename, content.len());

    fs::write(filename, content)?;
    println!("File written successfully: {filename}");

    // ファイル情報表示
    if log::log_enabled!(log::Level::Debug) {
        let metadata = fs::metadata(filename)?;
        debug_file_metadata(&metadata);
    }

    Ok(())
}

/// 現在のディレクトリの内容を一覧表示する
///
/// # Errors
///
/// - ディレクトリの読み取り権限がない場合
pub fn handle_ls() -> Result<()> {
    debug!("Listing current directory contents");

    let current_dir = env::current_dir()?;
    debug!("Listing directory: {current_dir:?}");

    let entries = fs::read_dir(current_dir)?;
    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_str().unwrap_or("???");
        if path.is_dir() {
            println!("{name}/");
        } else {
            println!("{name}");
        }

        // ファイル情報表示
        if log::log_enabled!(log::Level::Debug) {
            let metadata = entry.metadata()?;
            debug_file_metadata(&metadata);
        }
    }

    Ok(())
}

/// 現在のディレクトリの内容を一覧表示する
///
/// # Errors
///
/// - ディレクトリが存在しない場合
/// - ディレクトリではなくファイルを指定した場合
/// - アクセス権限がない場合
pub fn handle_cd(path: &str) -> Result<()> {
    // 移動するディレクトリ
    let target_path = match path {
        // 前のディレクトリを取得
        "-" => match env::var("OLDPWD") {
            Ok(old) => old,
            Err(_) => {
                return Err(RucliError::InvalidArgument(
                    "cd: OLDPWD not set".to_string(),
                ));
            }
        },
        // ホームディレクトリを取得
        "~" => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
        // 通常のディレクトリを取得
        _ => path.to_string(),
    };

    // ディレクトリ変更前に現在の場所を保存
    let old_dir = env::current_dir()?;

    // ディレクトリ変更
    env::set_current_dir(&target_path)?;

    // ディレクトリ移動に成功したらOLDPWDを更新
    unsafe {
        env::set_var("OLDPWD", old_dir);
    }

    debug!("change directory to : {target_path}");

    Ok(())
}

/// 現在の作業ディレクトリを表示
///
/// # Errors
///
/// - 現在のディレクトリが削除されている場合
/// - アクセス権限がない場合
pub fn handle_pwd() -> Result<()> {
    debug!("output the current working directory");

    let current_dir = env::current_dir()?;
    println!("{}", current_dir.display());
    Ok(())
}

/// ディレクトリを作成する
///
/// # Errors
///
/// - 既にディレクトリが存在する場合
/// - 親ディレクトリが存在しない場合
/// - 書き込み権限がない場合
pub fn handle_mkdir(path: &str, parents: bool) -> Result<()> {
    debug!("Creating directory : {path}");

    if parents {
        fs::create_dir_all(path)?;
        info!("Created directory (with parents): {path}");
    } else {
        fs::create_dir(path)?;
        info!("Created directory: {path}");
    }
    Ok(())
}

/// ファイルを削除する
///
/// # Errors
///
/// - ファイルが存在しない場合
/// - ディレクトリを指定した場合
/// - 削除権限がない場合
pub fn handle_rm(path: &str) -> Result<()> {
    debug!("deleting file: {path}");

    fs::remove_file(path)?;
    info!("Deleted file: {path}");

    Ok(())
}

/// プログラムを終了する
pub fn handle_exit() {
    info!("Exiting rucli");
    println!("good bye");
    // 0が正常終了、1以上がエラー
    process::exit(0);
}
