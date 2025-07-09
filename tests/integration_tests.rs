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
        .write_stdin("write project/src/main.rs fn main() {}\ncat project/src/main.rs\nexit\n")
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

#[test]
fn test_append_redirect_basic() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("append_test.txt");

    // 一つのセッションで全て実行
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo First line > {}\n\
             echo Second line >> {}\n\
             echo Third line >> {}\n\
             cat {}\n\
             exit\n",
            file_path.display(),
            file_path.display(),
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("First line"))
        .stdout(predicate::str::contains("Second line"))
        .stdout(predicate::str::contains("Third line"));
}

#[test]
fn test_append_redirect_new_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("new_append.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo Hello new file >> {}\n\
             cat {}\n\
             exit\n",
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello new file"));
}

#[test]
fn test_append_redirect_with_pipeline() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("pipeline_append.txt");
    let data_file = temp_dir.path().join("data.txt");

    // writeコマンドで複数行は書けないので、echoで複数回書く
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo apple > {}\n\
             echo banana >> {}\n\
             echo apricot >> {}\n\
             echo blueberry >> {}\n\
             exit\n",
            data_file.display(),
            data_file.display(),
            data_file.display(),
            data_file.display()
        ))
        .assert()
        .success();

    // パイプラインとリダイレクトのテスト
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "cat {} | grep a >> {}\n\
             cat {} | grep b >> {}\n\
             cat {}\n\
             exit\n",
            data_file.display(),
            file_path.display(),
            data_file.display(),
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("apple"))
        .stdout(predicate::str::contains("banana"))
        .stdout(predicate::str::contains("apricot"))
        .stdout(predicate::str::contains("blueberry"));
}

#[test]
fn test_append_redirect_empty_output() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("empty_append.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo Initial content > {}\n\
             echo hello | grep xyz >> {}\n\
             cat {}\n\
             exit\n",
            file_path.display(),
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("Initial content"))
        .stdout(predicate::str::contains("hello").not());
}

#[test]
fn test_redirect_overwrite_vs_append() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("compare.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo Line 1 > {}\n\
             echo Line 2 >> {}\n\
             echo Line 3 > {}\n\
             cat {}\n\
             exit\n",
            file_path.display(),
            file_path.display(),
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("Line 3"))
        .stdout(predicate::str::contains("Line 1").not())
        .stdout(predicate::str::contains("Line 2").not());
}

#[test]
fn test_input_redirect_basic() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("input.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo Hello, Input Redirect! > {}\n\
             cat < {}\n\
             exit\n",
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Input Redirect!"));
}

#[test]
fn test_input_redirect_with_grep() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("data.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo apple > {}\n\
             echo banana >> {}\n\
             echo apricot >> {}\n\
             grep a < {}\n\
             exit\n",
            file_path.display(),
            file_path.display(),
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("apple"))
        .stdout(predicate::str::contains("banana"))
        .stdout(predicate::str::contains("apricot"));
}

#[test]
fn test_input_redirect_nonexistent_file() {
    let temp_dir = tempfile::tempdir().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin("cat < nonexistent.txt\nexit\n")
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file or directory"));
}

#[test]
fn test_input_redirect_with_pipeline() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("pipeline_test.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo hello world > {}\n\
             echo hello rust >> {}\n\
             grep hello < {} | grep world\n\
             exit\n",
            file_path.display(),
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn test_background_execution_immediate_return() {
    let mut cmd = Command::cargo_bin("rucli").unwrap();
    let start = std::time::Instant::now();

    // 3秒のsleepをバックグラウンドで実行
    cmd.write_stdin("sleep 3 &\nexit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("[1]"));

    // 1秒以内に終了することを確認（バックグラウンドなので待たない）
    let duration = start.elapsed();
    assert!(duration.as_secs() < 2);
}

#[test]
fn test_background_with_output() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("bg_test.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "write {} background content &\n\
             sleep 1\n\
             cat {}\n\
             exit\n",
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("[1]"))
        .stdout(predicate::str::contains("background content"));
}

#[test]
fn test_multiple_background_jobs() {
    Command::cargo_bin("rucli")
        .unwrap()
        .write_stdin(
            "echo first &\n\
             echo second &\n\
             echo third &\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("[1]"))
        .stdout(predicate::str::contains("[2]"))
        .stdout(predicate::str::contains("[3]"));
}

#[test]
fn test_background_with_pipeline() {
    let temp_dir = tempfile::tempdir().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "echo hello world | grep hello &\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("[1]"));
}

#[test]
fn test_background_with_redirect() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("output.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "echo background test > {} &\n\
             sleep 1\n\
             cat {}\n\
             exit\n",
            file_path.display(),
            file_path.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("[1]"))
        .stdout(predicate::str::contains("background test"));
}
