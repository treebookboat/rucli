//! Phase 2 統合テスト
//! 複数のコマンドを組み合わせた実践的なワークフローをテスト

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_file_operations_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // 1. ファイル作成と確認を一つのセッションで
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("write test.txt Hello, World!\ncat test.txt\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("File written successfully"))
        .stdout(predicate::str::contains("Hello, World!"));

    // 2. ファイルのコピーと確認
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("cp test.txt backup.txt\ncat backup.txt\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));

    // 3. ファイルの移動
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("mv backup.txt archive.txt\nexit\n")
        .assert()
        .success();

    // 4. 移動後の確認（元のファイルは存在しない）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("cat backup.txt\nexit\n")
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    // 5. findでファイル検索
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("find *.txt\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        .stdout(predicate::str::contains("archive.txt"));

    // 6. grepで内容検索
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("grep Hello test.txt archive.txt\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("test.txt:1: Hello, World!"))
        .stdout(predicate::str::contains("archive.txt:1: Hello, World!"));
}

#[test]
fn test_directory_operations_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // 1. 階層的なディレクトリ作成
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("mkdir -p project/src/modules\nexit\n")
        .assert()
        .success();

    // 2. ディレクトリ移動とファイル作成
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("cd project/src\nwrite main.rs fn main() {}\ncat main.rs\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("fn main() {}"));

    // 3. pwdで現在位置確認
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("cd project/src\npwd\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("project/src"));

    // 4. ディレクトリのコピー
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("cp -r project backup_project\nexit\n")
        .assert()
        .success();

    // 5. findでディレクトリ内のファイル検索
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("find project *.rs\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("main.rs"));

    // 6. ディレクトリの再帰的削除
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("rm -rf project\nrm -rf backup_project\nexit\n")
        .assert()
        .success();
}

#[test]
fn test_alias_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // エイリアスの設定と使用を一つのセッションで
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("write test.txt content\nalias ll=ls\nalias\nll\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("ll = ls"))
        .stdout(predicate::str::contains("test.txt"));
}

#[test]
fn test_error_handling_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // 各エラーケースを個別にテスト

    // 1. 存在しないファイルのcat
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("cat nonexistent.txt\nexit\n")
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file"));

    // 2. ディレクトリのcat（エラー）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("mkdir testdir\ncat testdir\nexit\n")
        .assert()
        .success()
        .stderr(predicate::str::contains("is a directory"));

    // 3. 無効な正規表現でのgrep
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("write test.txt content\ngrep [invalid test.txt\nexit\n")
        .assert()
        .success()
        .stderr(predicate::str::contains("Invalid syntax"));

    // 4. rm -fで存在しないファイル（エラー無視）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("rm -f nonexistent.txt\nexit\n")
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_wildcard_patterns() {
    let temp_dir = TempDir::new().unwrap();

    // テストファイルの準備（一つのセッションで作成）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "write test.txt content\n\
             write test.rs code\n\
             write data.json {}\n\
             write README.md # Title\n\
             exit\n",
        )
        .assert()
        .success();

    // 1. アスタリスクパターン
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("find *.txt\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        .stdout(predicate::str::contains("data.json").not());

    // 2. 複合パターン（2文字の拡張子）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("find *.??\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("test.rs"))
        .stdout(predicate::str::contains("README.md"))
        .stdout(predicate::str::contains("test.txt").not());
}

#[test]
fn test_large_file_operations() {
    let temp_dir = TempDir::new().unwrap();

    // 大きめのファイルを作成
    let large_content = "Line of text\n".repeat(1000);
    fs::write(temp_dir.path().join("large.txt"), &large_content).unwrap();

    // 1. 大きなファイルのgrep（最初と最後の行を確認）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("grep Line large.txt\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Line of text"))
        .stdout(predicate::str::contains("1000: Line of text"));

    // 2. 大きなファイルのコピー
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("cp large.txt backup_large.txt\nexit\n")
        .assert()
        .success();

    // コピーされたファイルのサイズ確認
    let original_size = fs::metadata(temp_dir.path().join("large.txt"))
        .unwrap()
        .len();
    let backup_size = fs::metadata(temp_dir.path().join("backup_large.txt"))
        .unwrap()
        .len();
    assert_eq!(original_size, backup_size);
}

#[test]
fn test_version_command() {
    Command::cargo_bin("rucli")
        .unwrap()
        .write_stdin("version\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("rucli v"));
}
