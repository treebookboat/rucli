use rucli::commands::COMMANDS;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::TempDir;

#[test]
// helpコマンドの出力をテスト
fn test_help_command() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // helpコマンド実行
    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "help").unwrap();
    writeln!(stdin, "exit").unwrap();

    // 出力を取得
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // COMMANDSを使って検証
    for cmd_info in COMMANDS {
        assert!(
            stdout.contains(cmd_info.name),
            "Command '{}' not found in help output",
            cmd_info.name
        )
    }
}

#[test]
// echoコマンドの動作をテスト
fn test_echo_command() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // echoコマンド実行
    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "echo hello").unwrap();
    writeln!(stdin, "exit").unwrap();

    // 出力を取得
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // echoの出力が入っているか検証
    assert!(stdout.contains("hello"));
}

#[test]
// ファイル読み込みをテスト（テスト用ファイルを作成）
fn test_cat_command() {
    // テスト用ディレクトリを作成
    let temp_dir = TempDir::new().unwrap();

    // テスト用ファイルを作成
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Hello from file!").unwrap();

    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // catコマンド実行
    let stdin = child.stdin.as_mut().unwrap();
    let file_path_str = file_path.to_str().unwrap();
    writeln!(stdin, "cat {file_path_str}").unwrap();
    writeln!(stdin, "exit").unwrap();

    // 出力確認
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // ファイルの中身が出力されている確認
    assert!(stdout.contains("Hello from file!"));
}

#[test]
// ファイル書き込みをテスト
fn test_write_command() {
    // テスト用ディレクトリを作成
    let temp_dir = TempDir::new().unwrap();

    // テスト用ファイルを作成
    let file_path = temp_dir.path().join("test.txt");

    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // writeコマンド実行
    let stdin = child.stdin.as_mut().unwrap();
    let file_path_str = file_path.to_str().unwrap();
    writeln!(stdin, "write {file_path_str} Hello from write!").unwrap();
    writeln!(stdin, "exit").unwrap();

    // 出力確認
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // ファイルが存在しているか
    assert!(file_path.exists(), "File was not created");

    // ファイルの内容確認
    let contents = fs::read_to_string(&file_path).unwrap();
    assert_eq!(contents, "Hello from write!");

    // 成功メッセージの確認
    assert!(stdout.contains("File written successfully"));
}

#[test]
// 不正なコマンドのエラー処理
fn test_invalid_command() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // 不正なコマンド実行
    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "abc").unwrap();
    writeln!(stdin, "exit").unwrap();

    // 出力を取得
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // errorメッセージが出ているか確認
    assert!(stderr.contains("unknown command error"));
}

#[test]
// lsコマンドのテスト
fn test_ls_command() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "ls").unwrap();
    writeln!(stdin, "exit").unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // プロジェクトに必ず存在するファイルを確認
    assert!(stdout.contains("Cargo.toml"));
    assert!(stdout.contains("src/")); // ディレクトリ
}

#[test]
// repeatコマンドのテスト
fn test_repeat_command() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "repeat 3 hello").unwrap();
    writeln!(stdin, "exit").unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    let hello_count = stdout.matches("hello").count();
    assert_eq!(hello_count, 3);
}

#[test]
fn test_repeat_invalid_count() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "repeat -1 test").unwrap();
    writeln!(stdin, "repeat abc test").unwrap();
    writeln!(stdin, "quit").unwrap();

    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("must be positive"));
    assert!(stderr.contains("isn't a valid number"));
}

#[test]
// exitコマンドで正常終了
fn test_exit_command() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "exit").unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
}

#[test]
// quitコマンドで正常終了
fn test_quit_command() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "quit").unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
}

#[test]
// 複数コマンドの連続実行
fn test_multiple_commands() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // echoコマンド実行
    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "echo first").unwrap();
    writeln!(stdin, "echo second").unwrap();
    writeln!(stdin, "echo third").unwrap();
    writeln!(stdin, "exit").unwrap();

    // 出力を取得
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // echoの出力が入っているか検証
    assert!(stdout.contains("first"));
    assert!(stdout.contains("second"));
    assert!(stdout.contains("third"));
}
