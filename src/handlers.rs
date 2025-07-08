//! 各コマンドの実装を提供するモジュール

use crate::error::{Result, RucliError};
use log::{debug, info, warn};
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::{env, fs, io, os::unix::fs::PermissionsExt, path::Path, process};

use crate::commands::COMMANDS;
use crate::parser::{DEFAULT_HOME_INDICATOR, PREVIOUS_DIR_INDICATOR};

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
        PREVIOUS_DIR_INDICATOR => match env::var("OLDPWD") {
            Ok(old) => old,
            Err(_) => {
                return Err(RucliError::InvalidArgument(
                    "cd: OLDPWD not set".to_string(),
                ));
            }
        },
        // ホームディレクトリを取得
        DEFAULT_HOME_INDICATOR => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
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

/// ファイル/ディレクトリを削除する
///
/// # Errors
///
/// - ファイルが存在しない場合
/// - ディレクトリを指定した場合
/// - 削除権限がない場合
pub fn handle_rm(path: &str, recursive: bool, force: bool) -> Result<()> {
    debug!("deleting file: {path}");

    let result = if recursive {
        fs::remove_dir_all(path).or_else(|_| fs::remove_file(path))
    } else {
        fs::remove_file(path)
    };

    match result {
        Ok(()) => {
            info!("Deleted file: {path}");
            Ok(())
        }
        Err(e) => {
            if force {
                debug!("force mode : ignoring error - {e}");
                Ok(())
            } else {
                Err(RucliError::IoError(e))
            }
        }
    }
}

/// ファイルをコピーする
///
/// # Errors
///
/// - ソースファイルが存在しない場合
/// - ソースがディレクトリの場合
/// - 書き込み権限がない場合
pub fn handle_cp(source: &str, destination: &str, recursive: bool) -> Result<()> {
    debug!("Copying {source} to {destination}");

    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    // -rオプションがない状態ではディレクトリのコピーはできない
    if !recursive && source_path.is_dir() {
        return Err(RucliError::InvalidArgument(
            "source is a directory (use -r for recursive copy)".to_string(),
        ));
    }

    let bytes = if recursive {
        copy_dir_recursive(source_path, destination_path)?
    } else {
        // destinationがディレクトリであればディレクトリの先にコピー
        let destination_path = if destination_path.is_dir() {
            destination_path.join(source_path.file_name().unwrap())
        } else {
            destination_path.to_path_buf()
        };

        fs::copy(source_path, destination_path)?
    };

    info!("Copied {bytes} bytes from {source} to {destination}");

    Ok(())
}

// 再帰的なコピーを行う
fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<u64> {
    // 合計のバイト数
    let mut bytes = 0;

    // destinationがディレクトリの場合、まず作成
    if !destination.exists() {
        fs::create_dir(destination)?;
    }

    let entries = fs::read_dir(source)?;

    for entry in entries {
        debug!("now source directory : {entry:?}");

        let entry = entry?;

        // ディレクトリであれば新しいディレクトリを作成し、再帰的に関数を呼ぶ
        if entry.path().is_dir() {
            let new_source: std::path::PathBuf = entry.path();
            let new_destination = Path::new(destination).join(entry.file_name());

            // 新しいディレクトリを作成
            fs::create_dir(&new_destination)?;

            bytes += copy_dir_recursive(&new_source, &new_destination)?;
        }
        // ファイルなのでコピーをする
        else {
            let new_source: std::path::PathBuf = entry.path();
            let new_destination = Path::new(destination).join(entry.file_name());

            bytes += fs::copy(new_source, new_destination)?;
        }
    }

    Ok(bytes)
}

/// ファイルまたはディレクトリを移動・リネームする
///
/// # Arguments
///
/// * `source` - 移動元のファイルまたはディレクトリのパス
/// * `destination` - 移動先のパス
///
/// # Errors
///
/// - ソースが存在しない場合
/// - 移動先に書き込み権限がない場合
/// - クロスデバイス移動でコピーに失敗した場合
pub fn handle_mv(source: &str, destination: &str) -> Result<()> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    // ファイル->ディレクトリの時はディレクトリ内にファイルを移動
    let destination_path = if source_path.is_file() && destination_path.is_dir() {
        destination_path.join(source_path.file_name().unwrap())
    } else {
        destination_path.to_path_buf()
    };

    fs::rename(source_path, destination_path)?;
    Ok(())
}

/// ファイルを名前で検索する（ワイルドカード対応）
///
/// # Arguments
///
/// * `path` - 検索を開始するディレクトリ（Noneの場合はカレントディレクトリ）
/// * `pattern` - 検索パターン（ワイルドカード: *, ? を使用可能）
///
/// # Errors
///
/// - 検索開始ディレクトリが存在しない場合
/// - ディレクトリの読み取り権限がない場合
pub fn handle_find(path: Option<&str>, name: &str) -> Result<()> {
    let search_path = path.unwrap_or(".");

    let entries = fs::read_dir(search_path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        // ファイル名が一致すればパスを出力
        if let Some(filename) = entry_path.file_name().and_then(|n| n.to_str()) {
            if matches_pattern(filename, name) {
                println!("{}", entry_path.display());
            }
        }

        // ディレクトリであれば再帰的に探索
        if entry_path.is_dir() {
            handle_find(entry_path.to_str(), name)?;
        }
    }

    Ok(())
}

/// パターンがファイル名にマッチするかチェック
fn matches_pattern(filename: &str, pattern: &str) -> bool {
    match_helper(filename.as_bytes(), pattern.as_bytes(), 0, 0)
}

fn match_helper(filename: &[u8], pattern: &[u8], fi: usize, pi: usize) -> bool {
    // 両方終わった → OK
    if pi >= pattern.len() && fi >= filename.len() {
        return true;
    }

    // パターンだけ終わった → NG
    if pi >= pattern.len() {
        return false;
    }

    // ファイル名だけ終わったなら残りが全部*ならOK
    if fi >= filename.len() {
        return pattern[pi..].iter().all(|&c| c == b'*');
    }

    match pattern[pi] {
        b'?' => {
            // ファイルの長さがパターンより短いとダメ
            match_helper(filename, pattern, fi + 1, pi + 1)
        }
        b'*' => {
            // *を消して次の文字と比較
            if match_helper(filename, pattern, fi, pi + 1) {
                return true;
            }

            // *の文字を増やす
            if match_helper(filename, pattern, fi + 1, pi) {
                return true;
            }

            // 多分ここまで来ないはず
            false
        }
        _ => {
            if pattern[pi] == filename[fi] {
                match_helper(filename, pattern, fi + 1, pi + 1)
            } else {
                false
            }
        }
    }
}

/// ファイル内でパターンを検索する
///
/// # Arguments
///
/// * `pattern` - 検索する文字列パターン
/// * `files` - 検索対象のファイルパス一覧
///
/// # Errors
///
/// - ファイルが存在しない場合
/// - ファイルの読み取り権限がない場合
pub fn handle_grep(pattern: &str, files: &[String]) -> Result<()> {
    for file in files {
        let results = grep_file(pattern, file)?;

        if results.is_empty() {
            continue;
        }

        for (line_num, content) in results {
            if files.len() > 1 {
                println!("{}:{}: {}", file, line_num + 1, content);
            } else {
                println!("{}: {}", line_num + 1, content);
            }
        }
    }

    Ok(())
}

/// 単一ファイルを検索
fn grep_file(pattern: &str, filepath: &str) -> Result<Vec<(usize, String)>> {
    // 1. 最初に一度だけ正規表現をコンパイル
    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Err(RucliError::InvalidRegex(e.to_string())),
    };

    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut results = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if re.is_match(&line) {
            results.push((line_num, line));
        }
    }

    Ok(results)
}

/// プログラムを終了する
pub fn handle_exit() {
    info!("Exiting rucli");
    println!("good bye");
    // 0が正常終了、1以上がエラー
    process::exit(0);
}
