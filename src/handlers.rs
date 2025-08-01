//! 各コマンドの実装を提供するモジュール

use crate::alias::{list_aliases, set_alias};
use crate::environment::{get_var, list_all_vars, set_var};
use crate::error::{Result, RucliError};
use crate::history::{get_history_by_number, get_history_list, search_history};
use crate::{functions, job};
use log::{debug, info, warn};
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::{env, fs, io, os::unix::fs::PermissionsExt, path::Path};

use crate::commands::{
    COMMANDS, Command, CommandResult, EnvironmentAction, HistoryAction, execute_command,
    execute_command_internal,
};
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

/// メッセージを文字列として返す
///
/// # Arguments
///
/// * `message` - 表示するメッセージ
///
/// # Returns
///
/// メッセージの文字列
pub fn handle_echo(message: &str) -> String {
    message.to_string()
}

/// ヘルプメッセージを表示する
pub fn handle_help() -> String {
    let mut lines = Vec::new();

    lines.push("Available commands:".to_string());

    // 左寄せでそろえるために最長のusageを計算
    let max_width = COMMANDS
        .iter()
        .map(|cmd| cmd.usage.len())
        .max()
        .unwrap_or(0);

    for cmd in COMMANDS {
        // cmd.usage と cmd.description を表示
        lines.push(format!(
            "  {:<width$} - {}",
            cmd.usage,
            cmd.description,
            width = max_width
        ))
    }

    lines.push("Options:".to_string());
    lines.push("  --debug    Enable debug mode with detailed logging".to_string());

    lines.join("\n")
}

/// 文字列をcount回表示
pub fn handle_repeat(count: i32, message: &str) -> String {
    let mut lines = Vec::new();
    for _ in 0..count {
        lines.push(message);
    }
    lines.join("\n")
}

/// ファイルの内容を表示する
///
/// # Errors
///
/// - ファイルが存在しない場合
/// - ディレクトリを指定した場合
/// - 読み取り権限がない場合
pub fn handle_cat(filename: &str, input: Option<&str>) -> Result<String> {
    // inputがある場合は標準入力として扱う
    if let Some(input_content) = input {
        return Ok(input_content.to_string());
    }

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

    // ファイル読み込み成功時
    info!("Successfully read file: {filename}");

    Ok(contents)
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
pub fn handle_ls() -> Result<String> {
    debug!("Listing current directory contents");

    let current_dir = env::current_dir()?;
    debug!("Listing directory: {current_dir:?}");

    // 出力する文字列の集合
    let mut lines = Vec::new();

    let entries = fs::read_dir(current_dir)?;
    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_str().unwrap_or("???");
        if path.is_dir() {
            lines.push(format!("{name}/"));
        } else {
            lines.push(name.to_string());
        }

        // ファイル情報表示
        if log::log_enabled!(log::Level::Debug) {
            let metadata = entry.metadata()?;
            debug_file_metadata(&metadata);
        }
    }

    Ok(lines.join("\n"))
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
pub fn handle_pwd() -> Result<String> {
    debug!("output the current working directory");

    let current_dir = env::current_dir()?;
    Ok(format!("{}", current_dir.display()))
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
pub fn handle_find(path: Option<&str>, name: &str) -> Result<String> {
    let mut lines = Vec::new();

    let search_path = path.unwrap_or(".");

    let entries = fs::read_dir(search_path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        // ファイル名が一致すればパスを出力
        if let Some(filename) = entry_path.file_name().and_then(|n| n.to_str())
            && matches_pattern(filename, name)
        {
            lines.push(format!("{}", entry_path.display()));
        }

        // ディレクトリであれば再帰的に探索
        if entry_path.is_dir() {
            let sub_results = handle_find(entry_path.to_str(), name)?;
            if !sub_results.is_empty() {
                lines.push(sub_results);
            }
        }
    }

    Ok(lines.join("\n"))
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
pub fn handle_grep(pattern: &str, files: &[String], input: Option<&str>) -> Result<String> {
    let mut lines = Vec::new();

    if files.is_empty() {
        if let Some(input_text) = input {
            // パイプラインからの入力を処理
            let results = grep_from_string(pattern, input_text)?;

            for (line_num, content) in results {
                if input.is_some() {
                    lines.push(content);
                } else {
                    lines.push(format!("{}: {}", line_num + 1, content));
                }
            }
        }
    } else {
        // 既存のファイル処理
        for file in files {
            let results = grep_file(pattern, file)?;

            if results.is_empty() {
                continue;
            }

            for (line_num, content) in results {
                if files.len() > 1 {
                    lines.push(format!("{}:{}: {}", file, line_num + 1, content));
                } else {
                    lines.push(format!("{}: {}", line_num + 1, content));
                }
            }
        }
    }

    Ok(lines.join("\n"))
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

fn grep_from_string(pattern: &str, text: &str) -> Result<Vec<(usize, String)>> {
    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Err(RucliError::InvalidRegex(e.to_string())),
    };

    let mut results = Vec::new();

    for (line_num, line) in text.lines().enumerate() {
        if re.is_match(line) {
            results.push((line_num, line.to_string()));
        }
    }

    Ok(results)
}

// handlers.rs に追加
/// コマンドエイリアスを管理する
///
/// # Arguments
///
/// * `name` - エイリアス名（Noneの場合は一覧表示）
/// * `command` - エイリアスに設定するコマンド
///
/// # Errors
///
/// - 無効なエイリアス名の場合
pub fn handle_alias(name: Option<&str>, command: Option<&str>) -> Result<()> {
    match (name, command) {
        (None, None) => {
            // ALIASESから全て取得して一覧表示
            for (name, cmd) in list_aliases() {
                println!("{name} = {cmd}");
            }
        }
        (Some(name), Some(cmd)) => {
            set_alias(name, cmd);
        }
        _ => {
            // このパターンは来ないはず（パーサーで防いでいる）
            unreachable!()
        }
    }

    Ok(())
}

/// バックグラウンド実行
pub fn handle_background_execution(command: Box<Command>) -> Result<String> {
    // 表示用のコマンド文字列
    let cmd_str = format!("{command:?}");

    let job_id = job::get_next_job_id();

    // スレッドを起動
    let handle = thread::spawn(move || {
        // ここで実際にコマンドが実行される（遅延）
        if let Err(e) = execute_command(*command, None) {
            eprintln!("Background job failed: {e}");
        }
        // 完了を通知
        job::mark_completed(job_id);
    });

    // スレッドIDを取得
    let thread_id = handle.thread().id();

    // ジョブ作成
    job::create_job_with_id(job_id, cmd_str, thread_id);

    // ユーザーに通知
    Ok(format!("[{job_id}] {thread_id:?}"))
}

/// バージョン情報を表示する
pub fn handle_version() -> String {
    format!("rucli v{}", env!("CARGO_PKG_VERSION"))
}

/// 一定秒数スリープ
pub fn handle_sleep(seconds: u64) -> Result<()> {
    thread::sleep(Duration::from_secs(seconds));
    Ok(())
}

/// ジョブ一覧表示
pub fn handle_jobs() -> Result<String> {
    // ジョブのリストを取得
    let jobs = job::list_jobs();

    // 何も入っていない
    if jobs.is_empty() {
        return Ok("No jobs".to_string());
    }

    // 3. 表示用の文字列を構築
    let mut lines = Vec::new();
    let last_idx = jobs.len() - 1;

    // 4. 各ジョブをフォーマット
    for (i, job) in jobs.iter().enumerate() {
        // [1]+ Running    sleep 10
        // [2]- Running    sleep 5
        // [3]  Running    echo hello

        let marker = if i == last_idx {
            "+"
        } else if i == last_idx - 1 {
            "-"
        } else {
            " "
        };

        // 実際のステータスを表示
        let status = match job.status {
            job::JobStatus::Running => "Running",
            job::JobStatus::Completed => "Done", // 通常は表示されないが念のため
        };

        lines.push(format!(
            "[{}]{} {:10} {}",
            job.id,      // [1]
            marker,      // +
            status,      // status   (10文字幅)
            job.command  // sleep 10
        ));
    }

    Ok(lines.join("\n"))
}

/// フォアグラウンド変更
pub fn handle_fg(job_id: Option<u32>) -> Result<()> {
    // 1. 対象ジョブの決定
    let target_id = match job_id {
        Some(id) => id,
        None => {
            // 最新のジョブIDを取得
            let jobs = job::list_jobs();
            if jobs.is_empty() {
                return Err(RucliError::InvalidArgument("No jobs".to_string()));
            }
            jobs.last().unwrap().id
        }
    };

    // 2. ジョブを取得
    match job::get_job(target_id) {
        Some(job) => {
            // 3. 状態を表示
            println!("Job [{}] ({}) is still running", job.id, job.command);
            // 将来: ここで待機処理
            Ok(())
        }
        None => Err(RucliError::InvalidArgument(format!(
            "No such job: {target_id}"
        ))),
    }
}

/// 環境変数コマンドのハンドラ
pub fn handle_environment(action: EnvironmentAction) -> Result<String> {
    let mut lines = Vec::new();

    match action {
        EnvironmentAction::List => {
            let env_list = list_all_vars();
            for (name, value) in env_list {
                lines.push(format!("{name}={value}"));
            }
            Ok(lines.join("\n"))
        }
        EnvironmentAction::Show(var_name) => {
            if let Some(value) = get_var(&var_name) {
                Ok(value)
            } else {
                Err(RucliError::InvalidArgument(format!(
                    "Environment variable '{var_name}' not found"
                )))
            }
        }
        EnvironmentAction::Set(var_name, value) => {
            set_var(var_name.as_str(), value.as_str());
            Ok(String::new())
        }
    }
}

/// 関数を定義する
///
/// # Arguments
/// * `name` - 関数名
/// * `body` - 関数本体のコマンド
///
pub fn handle_function_definition(name: &str, body: Command) -> Result<()> {
    functions::define_function(name, body);
    Ok(())
}

/// 関数を実行する
///
/// # Arguments
/// * `name` - 関数名
/// * `args` - 関数に渡す引数
///
/// # Returns
/// * 関数の実行結果（文字列）
///
pub fn handle_function_call(name: &str, args: &[String]) -> Result<String> {
    if let Some(cmd) = functions::get_function(name) {
        // 引数の設定
        for (i, arg) in args.iter().enumerate() {
            let var_name = (i + 1).to_string(); // "1", "2", ...
            unsafe {
                std::env::set_var(&var_name, arg);
            }
        }

        let cmd_str = match execute_command_internal(cmd, None)? {
            CommandResult::Continue(output) => output,
            CommandResult::Exit => {
                // 関数内でのExitは無視して空文字列を返す
                String::new()
            }
        };

        // 引数のクリーンアップ
        for i in 0..args.len() {
            let var_name = (i + 1).to_string();
            unsafe {
                std::env::remove_var(&var_name);
            }
        }

        Ok(cmd_str)
    } else {
        Err(RucliError::UnknownCommand(format!(
            "function '{name}' not found"
        )))
    }
}

/// 履歴コマンドのハンドラー
pub fn handle_history(action: HistoryAction) -> Result<String> {
    match action {
        HistoryAction::List => {
            let list = get_history_list();

            Ok(list
                .iter()
                .map(|(num, cmd)| format!("{num:4}  {cmd}"))
                .collect::<Vec<_>>()
                .join("\n"))
        }
        HistoryAction::Search(query) => {
            let list = search_history(&query);

            Ok(list
                .iter()
                .map(|(num, cmd)| format!("{num:4}  {cmd}"))
                .collect::<Vec<_>>()
                .join("\n"))
        }
        HistoryAction::Execute(index) => match get_history_by_number(index) {
            Some(cmd) => Ok(cmd),
            None => Err(RucliError::InvalidArgument(format!(
                "history: {index}: history position out of range",
            ))),
        },
    }
}

/// プログラムを終了する
pub fn handle_exit() {
    info!("Exiting rucli");
    println!("good bye");
}
