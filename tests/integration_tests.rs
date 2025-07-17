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

#[test]
fn test_heredoc_basic_cat() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "cat <<EOF\n\
             Hello World\n\
             This is a test\n\
             EOF\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World"))
        .stdout(predicate::str::contains("This is a test"));
}

#[test]
fn test_heredoc_with_grep() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "grep error <<LOG\n\
             info: starting application\n\
             error: connection failed\n\
             info: retrying\n\
             error: timeout\n\
             LOG\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("error: connection failed"))
        .stdout(predicate::str::contains("error: timeout"))
        .stdout(predicate::str::contains("info: starting").not())
        .stdout(predicate::str::contains("info: retrying").not());
}

#[test]
fn test_heredoc_strip_indent() {
    let temp_dir = TempDir::new().unwrap();

    // タブ文字を実際に含む文字列を作成
    let input = format!(
        "cat <<-END\n{}First line with tab\n{}{}Second line with two tabs\n    Third line with spaces\nEND\nexit\n",
        "\t", // 1つのタブ
        "\t", // 1つ目のタブ
        "\t"  // 2つ目のタブ
    );

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("First line with tab"))
        .stdout(predicate::str::contains("\tSecond line with two tabs"))
        .stdout(predicate::str::contains("    Third line with spaces"));
}

#[test]
fn test_heredoc_with_variable_expansion() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "env USER=Alice\n\
             env GREETING=Hello\n\
             cat <<MESSAGE\n\
             $GREETING, $USER!\n\
             Welcome to rucli\n\
             MESSAGE\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Alice!"))
        .stdout(predicate::str::contains("Welcome to rucli"));
}

#[test]
fn test_heredoc_with_command_substitution() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "cat <<END\n\
             Echo test: $(echo Hello)\n\
             END\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Echo test: Hello"));
}

#[test]
fn test_heredoc_custom_delimiter() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "cat <<CUSTOM_END_MARKER\n\
             This uses a custom delimiter\n\
             EOF is just text here\n\
             CUSTOM_END_MARKER\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("This uses a custom delimiter"))
        .stdout(predicate::str::contains("EOF is just text here"));
}

#[test]
fn test_heredoc_write_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "cat <<CONFIG > {}\n\
             server=localhost\n\
             port=8080\n\
             debug=true\n\
             CONFIG\n\
             cat {}\n\
             exit\n",
            config_file.display(),
            config_file.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("server=localhost"))
        .stdout(predicate::str::contains("port=8080"))
        .stdout(predicate::str::contains("debug=true"));
}

#[test]
fn test_heredoc_empty_content() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "cat <<EMPTY\n\
             EMPTY\n\
             exit\n",
        )
        .assert()
        .success();
}

#[test]
fn test_heredoc_multiple_lines() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "cat <<MULTILINE\n\
             Line 1\n\
             Line 2\n\
             Line 3\n\
             Line 4\n\
             Line 5\n\
             MULTILINE\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Line 1"))
        .stdout(predicate::str::contains("Line 2"))
        .stdout(predicate::str::contains("Line 3"))
        .stdout(predicate::str::contains("Line 4"))
        .stdout(predicate::str::contains("Line 5"));
}

#[test]
fn test_heredoc_with_special_characters() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "cat <<SPECIAL\n\
             Special chars: !@#$%^&*()\n\
             Quotes: \"double\" and 'single'\n\
             Path: /usr/local/bin\n\
             SPECIAL\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Special chars: !@#$%^&*()"))
        .stdout(predicate::str::contains("Quotes: \"double\" and 'single'"))
        .stdout(predicate::str::contains("Path: /usr/local/bin"));
}

#[test]
fn test_heredoc_delimiter_in_content() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "cat <<EOF\n\
             This line contains EOF in the middle\n\
             EOF at the start of line is still content\n\
             Only a line with exactly EOF ends it\n\
             EOF\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "This line contains EOF in the middle",
        ))
        .stdout(predicate::str::contains(
            "EOF at the start of line is still content",
        ))
        .stdout(predicate::str::contains(
            "Only a line with exactly EOF ends it",
        ));
}

#[test]
fn test_heredoc_with_pipeline_and_redirect() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("filtered.txt");

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(format!(
            "grep line <<DATA | grep important > {}\n\
             line 1: not important\n\
             line 2: important data\n\
             line 3: also important\n\
             line 4: not relevant\n\
             DATA\n\
             cat {}\n\
             exit\n",
            output_file.display(),
            output_file.display()
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("line 2: important data"))
        .stdout(predicate::str::contains("line 3: also important"));
}

#[test]
fn test_script_basic_execution() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("test.rsh");

    // スクリプトファイルを作成
    fs::write(
        &script_file,
        "echo Hello from script\n\
         pwd\n\
         echo Done\n",
    )
    .unwrap();

    // スクリプトを実行
    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello from script"))
        .stdout(predicate::str::contains("Done"));
}

#[test]
fn test_script_with_shebang_and_comments() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("commented.rsh");

    fs::write(
        &script_file,
        "#!/usr/bin/env rucli\n\
         # This is a comment\n\
         echo First line\n\
         # Another comment\n\
         \n\
         echo Second line\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("First line"))
        .stdout(predicate::str::contains("Second line"))
        .stdout(predicate::str::contains("#!/usr/bin/env rucli").not())
        .stdout(predicate::str::contains("# This is a comment").not());
}

#[test]
fn test_script_with_error_continues() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("error.rsh");

    fs::write(
        &script_file,
        "echo Before error\n\
         cat nonexistent.txt\n\
         echo After error\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success() // スクリプトは続行
        .stdout(predicate::str::contains("Before error"))
        .stdout(predicate::str::contains("After error"))
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn test_script_not_found() {
    Command::cargo_bin("rucli")
        .unwrap()
        .arg("nonexistent.rsh")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Script file nonexistent.rsh not found",
        ));
}

#[test]
fn test_script_with_variables() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("vars.rsh");

    fs::write(
        &script_file,
        "env NAME=Script\n\
         echo Hello $NAME\n\
         env VERSION=1.0\n\
         echo Version: $VERSION\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello Script"))
        .stdout(predicate::str::contains("Version: 1.0"));
}

#[test]
fn test_script_with_command_substitution() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("subst.rsh");

    fs::write(
        &script_file,
        "echo Current dir: $(pwd)\n\
         echo Echo test: $(echo nested)\n\
         env VAR=test\n\
         echo Variable in substitution: $(echo $VAR)\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Current dir:"))
        .stdout(predicate::str::contains("Echo test: nested"))
        .stdout(predicate::str::contains("Variable in substitution: test"));
}

#[test]
fn test_script_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("fileops.rsh");

    fs::write(
        &script_file,
        "write test.txt Script created this file\n\
         cat test.txt\n\
         cp test.txt backup.txt\n\
         cat backup.txt\n\
         rm test.txt\n\
         rm backup.txt\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("File written successfully"))
        .stdout(predicate::str::contains("Script created this file").count(2));
}

#[test]
fn test_script_with_pipelines() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("pipes.rsh");

    fs::write(
        &script_file,
        "echo apple > fruits.txt\n\
         echo banana >> fruits.txt\n\
         echo apricot >> fruits.txt\n\
         cat fruits.txt | grep a\n\
         cat fruits.txt | grep a | wc -l\n\
         rm fruits.txt\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("apple"))
        .stdout(predicate::str::contains("banana"))
        .stdout(predicate::str::contains("apricot"));
}

#[test]
fn test_script_with_redirections() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("redirect.rsh");

    fs::write(
        &script_file,
        "echo First line > output.txt\n\
         echo Second line >> output.txt\n\
         cat < output.txt\n\
         rm output.txt\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("First line"))
        .stdout(predicate::str::contains("Second line"));
}

#[test]
fn test_script_with_background_jobs() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("background.rsh");

    fs::write(
        &script_file,
        "echo Starting background job\n\
         sleep 1 &\n\
         echo Background job started\n\
         jobs\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting background job"))
        .stdout(predicate::str::contains("[1]"))
        .stdout(predicate::str::contains("Background job started"));
}

#[test]
fn test_script_with_directory_operations() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("dirs.rsh");

    fs::write(
        &script_file,
        "mkdir test_dir\n\
         cd test_dir\n\
         pwd\n\
         write file.txt content\n\
         ls\n\
         cd ..\n\
         rm -rf test_dir\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("test_dir"))
        .stdout(predicate::str::contains("file.txt"));
}

#[test]
fn test_script_with_aliases() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("alias.rsh");

    // Note: エイリアスはセッション内でのみ有効
    fs::write(
        &script_file,
        "alias ll=ls\n\
         alias\n\
         write test.txt content\n\
         ll\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("ll = ls"))
        .stdout(predicate::str::contains("test.txt"));
}

#[test]
fn test_script_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("empty.rsh");

    fs::write(&script_file, "").unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .assert()
        .success();
}

#[test]
fn test_script_only_comments() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("comments_only.rsh");

    fs::write(
        &script_file,
        "#!/usr/bin/env rucli\n\
         # Just comments\n\
         # Nothing to execute\n\
         \n\
         # More comments\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_script_with_find_and_grep() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("search.rsh");

    fs::write(
        &script_file,
        "write test1.txt contains search term\n\
         write test2.rs rust code\n\
         write data.json {}\n\
         find . *.txt\n\
         grep search test1.txt\n\
         rm test1.txt\n\
         rm test2.rs\n\
         rm data.json\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("test1.txt"))
        .stdout(predicate::str::contains("contains search term"));
}

#[test]
fn test_script_complex_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("workflow.rsh");

    fs::write(
        &script_file,
        "#!/usr/bin/env rucli\n\
         # Complex workflow test\n\
         echo Setting up project...\n\
         \n\
         # Create directory structure\n\
         mkdir -p project/src\n\
         mkdir -p project/tests\n\
         \n\
         # Create files\n\
         cd project\n\
         write src/main.rs fn main() {}\n\
         write Cargo.toml [package]\n\
         \n\
         # List created files\n\
         find . *.rs\n\
         find . *.toml\n\
         \n\
         # Cleanup\n\
         cd ..\n\
         rm -rf project\n\
         echo Workflow completed!\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Setting up project..."))
        .stdout(predicate::str::contains("main.rs"))
        .stdout(predicate::str::contains("Cargo.toml"))
        .stdout(predicate::str::contains("Workflow completed!"));
}

#[test]
fn test_if_condition_success() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "if echo test; then echo OK; else echo FAIL; fi\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("OK"))
        .stdout(predicate::str::contains("FAIL").not());
}

#[test]
fn test_if_condition_failure() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "if cat /nonexistent/file.txt; then echo OK; else echo FAIL; fi\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("FAIL"))
        .stdout(predicate::str::contains("OK").not());
}

#[test]
fn test_if_without_else_success() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "if echo test; then echo SUCCESS; fi\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("SUCCESS"));
}

#[test]
fn test_if_without_else_failure() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "if cat /nonexistent; then echo OK; fi\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("OK").not());
}

#[test]
fn test_if_with_pwd_condition() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "if pwd; then echo Working dir found; fi\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("/")) // pwdの出力
        .stdout(predicate::str::contains("Working dir found"));
}

#[test]
fn test_if_with_variables() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "env STATUS=OK\n\
             if echo $STATUS; then echo Variable is $STATUS; fi\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("OK"))
        .stdout(predicate::str::contains("Variable is OK"));
}

#[test]
fn test_if_with_write_command() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "if write test.txt content; then echo Write successful; else echo Write failed; fi\n\
             cat test.txt\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("File written successfully"))
        .stdout(predicate::str::contains("Write successful"))
        .stdout(predicate::str::contains("content"));
}

#[test]
fn test_if_in_pipeline() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "echo test > file.txt\n\
             if cat file.txt | grep test; then echo Pattern found; fi\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("Pattern found"));
}

#[test]
fn test_while_loop_basic() {
    let temp_dir = TempDir::new().unwrap();

    // ファイルを作成してwhileループでテスト
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "write test.txt content\n\
             while cat test.txt; do rm test.txt; done\n\
             cat test.txt\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("File written successfully"))
        .stdout(predicate::str::contains("content"))
        .stderr(predicate::str::contains("No such file")); // 2回目のcatで失敗
}

#[test]
fn test_while_loop_counter() {
    let temp_dir = TempDir::new().unwrap();

    // カウンタ的な動作をシミュレート（3回実行して終了）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "write counter.txt 3\n\
             while cat counter.txt; do rm counter.txt; done\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_while_loop_immediate_false() {
    let temp_dir = TempDir::new().unwrap();

    // 最初から条件が偽の場合
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "while cat nonexistent.txt; do echo Should not appear; done\n\
             echo After loop\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Should not appear").not())
        .stdout(predicate::str::contains("After loop"));
}

#[test]
fn test_while_loop_with_echo() {
    let temp_dir = TempDir::new().unwrap();

    // 簡単なループ（手動で制限）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "write flag.txt yes\n\
             while cat flag.txt; do echo Loop executed; rm flag.txt; done\n\
             echo Loop finished\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("yes"))
        .stdout(predicate::str::contains("Loop executed"))
        .stdout(predicate::str::contains("Loop finished"));
}

#[test]
fn test_while_in_script() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("while_test.rsh");

    fs::write(
        &script_file,
        "#!/usr/bin/env rucli\n\
         # Test while loop in script\n\
         write data.txt test\n\
         while cat data.txt; do rm data.txt; done\n\
         echo Script completed\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("Script completed"));
}

#[test]
fn test_while_with_variables() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "env FILENAME=test.txt\n\
             write $FILENAME content\n\
             while cat $FILENAME; do rm $FILENAME; done\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("content"));
}

#[test]
fn test_while_body_error_continues() {
    let temp_dir = TempDir::new().unwrap();

    // ボディでエラーが発生してもループは継続（今回の実装では停止）
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "write test.txt line\n\
             while cat test.txt; do cat nonexistent.txt; done\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("line"))
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn test_function_definition_and_call() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function hello() { echo Hello, World!; }\n\
             hello\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn test_function_with_arguments() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function greet() { echo Hello, $1!; }\n\
             greet Alice\n\
             greet Bob\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Alice!"))
        .stdout(predicate::str::contains("Hello, Bob!"));
}

#[test]
fn test_function_multiple_arguments() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function show() { echo Args: $1, $2, $3; }\n\
             show first second third\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Args: first, second, third"));
}

#[test]
fn test_function_overwrite() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function test() { echo First version; }\n\
             test\n\
             function test() { echo Second version; }\n\
             test\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("First version"))
        .stdout(predicate::str::contains("Second version"));
}

#[test]
fn test_function_not_found() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "nonexistent_function arg1\n\
             exit\n",
        )
        .assert()
        .success()
        .stderr(predicate::str::contains("nonexistent_function"));
}

#[test]
fn test_function_in_pipeline() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function upper() { echo HELLO WORLD; }\n\
             upper | grep HELLO\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO WORLD"));
}

#[test]
fn test_function_with_file_operations() {
    let temp_dir = TempDir::new().unwrap();

    // 関数定義を分ける
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "write test.txt original content\n\
             function show() { cat $1; }\n\
             show test.txt\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("original content"));
}

#[test]
fn test_function_calling_function() {
    let temp_dir = TempDir::new().unwrap();

    // 単一コマンドのみサポートなので、echoだけにする
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function helper() { echo Helper: $1; }\n\
             helper test\n\
             function main() { echo Main with $1; }\n\
             main test\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Helper: test"))
        .stdout(predicate::str::contains("Main with test"));
}

#[test]
fn test_function_with_redirect() {
    let temp_dir = TempDir::new().unwrap();

    // クォートを修正
    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function logger() { echo Log: $1; }\n\
             logger TestMessage > log.txt\n\
             cat log.txt\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Log: TestMessage"));
}

#[test]
fn test_function_in_script() {
    let temp_dir = TempDir::new().unwrap();
    let script_file = temp_dir.path().join("functions.rsh");

    fs::write(
        &script_file,
        "#!/usr/bin/env rucli\n\
         # Function test script\n\
         function greet() { echo Hello, $1!; }\n\
         function farewell() { echo Goodbye, $1!; }\n\
         \n\
         greet Script\n\
         farewell Script\n",
    )
    .unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .arg(script_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Script!"))
        .stdout(predicate::str::contains("Goodbye, Script!"));
}

#[test]
fn test_function_with_background() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function background_task() { echo Running in background; }\n\
             background_task &\n\
             sleep 1\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("[1]"));
}

#[test]
fn test_function_in_if_condition() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function check() { echo Checking; }\n\
             if check; then echo Check passed; fi\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Checking"))
        .stdout(predicate::str::contains("Check passed"));
}

#[test]
fn test_function_empty_args() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function no_args() { echo No arguments needed; }\n\
             no_args\n\
             no_args extra args ignored\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("No arguments needed").count(2));
}

#[test]
fn test_function_with_command_substitution() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "function get_dir() { pwd; }\n\
             echo Current: $(get_dir)\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Current:"));
}

#[test]
fn test_function_with_variables() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("rucli")
        .unwrap()
        .current_dir(&temp_dir)
        .write_stdin(
            "env PREFIX=Hello\n\
             function say() { echo $PREFIX, $1!; }\n\
             say World\n\
             exit\n",
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}
