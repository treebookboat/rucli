//! 演算子（パイプ、リダイレクト、バックグラウンド等）のパース関数

/// 入力をパイプで分割する
/// 例: "echo hello | grep h" → ["echo hello", "grep h"]
pub fn split_by_pipe(input: &str) -> Vec<&str> {
    input
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect()
}

// リダイレクトでコマンドを分割
/// 例: "echo hello > file.txt" → ("echo hello", Some((">", "file.txt")))
pub(super) fn split_redirect(input: &str) -> (String, Option<(String, String)>) {
    if let Some((pos, redirect_op)) = find_redirect_position(input) {
        let command = input[..pos].trim().to_string();
        let target = input[pos + redirect_op.len()..].trim().to_string();

        if target.is_empty() {
            (input.to_string(), None)
        } else {
            (command, Some((redirect_op.to_string(), target)))
        }
    } else {
        (input.to_string(), None)
    }
}

// リダイレクト演算子を検出する共通関数
pub(super) fn find_redirect_position(input: &str) -> Option<(usize, &str)> {
    // ">>" を先にチェック（長い方を優先）
    if let Some(pos) = input.find(">>") {
        return Some((pos, ">>"));
    }
    // 次に ">" をチェック
    if let Some(pos) = input.find('>') {
        return Some((pos, ">"));
    }
    // 最後に "<" をチェック
    if let Some(pos) = input.find('<') {
        return Some((pos, "<"));
    }
    None
}

/// リダイレクトを含むかチェック
pub(super) fn contains_redirect(input: &str) -> bool {
    input.contains(">>") || input.contains(">") || input.contains("<")
}

// バックグラウンドを含むかチェック
pub(super) fn contains_background(input: &str) -> bool {
    input.contains("&")
}

/// ヒアドキュメントの情報を抽出
pub fn parse_heredoc_header(input: &str) -> Option<(String, String, bool)> {
    // "<<-"を探す(長いほうから)
    if let Some(pos) = input.find("<<-") {
        // コマンド部分をトリミング
        let cmd = input[..pos].trim();

        // デリミタ部分を取得
        let delimiter_part = input[pos + "<<-".len()..].trim();
        let delimiter = delimiter_part.split_whitespace().next()?;

        return Some((cmd.to_string(), delimiter.to_string(), true));
    }

    // "<<"を探す
    if let Some(pos) = input.find("<<") {
        // コマンド部分をトリミング
        let cmd = input[..pos].trim();

        // デリミタ部分を取得
        let delimiter_part = input[pos + "<<".len()..].trim();
        let delimiter = delimiter_part.split_whitespace().next()?;

        return Some((cmd.to_string(), delimiter.to_string(), false));
    }

    None
}

/// ヒアドキュメントを含むかチェック
pub fn contains_heredoc(input: &str) -> bool {
    input.contains("<<") && !input.contains("<<<")
}

/// 入力をセミコロンで分割する
pub(super) fn split_by_semicolon(input: &str) -> Vec<&str> {
    input
        .split(';')
        .map(|cmd| cmd.trim())
        .filter(|cmd| !cmd.is_empty()) // 空文字列を除外
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_by_pipe() {
        let input = "echo hello | grep h | wc -l";
        let parts = split_by_pipe(input);
        assert_eq!(parts, vec!["echo hello", "grep h", "wc -l"]);
    }

    #[test]
    fn test_split_by_pipe_with_spaces() {
        let input = "  echo hello  |  grep h  ";
        let parts = split_by_pipe(input);
        assert_eq!(parts, vec!["echo hello", "grep h"]);
    }

    #[test]
    fn test_split_by_pipe_empty_segments() {
        // 空のセグメントが除外されることを確認
        let input = "echo | | grep";
        let parts = split_by_pipe(input);
        assert_eq!(parts, vec!["echo", "grep"]);
    }

    #[test]
    fn test_split_by_pipe_single_command() {
        // パイプなしの場合
        let input = "echo hello world";
        let parts = split_by_pipe(input);
        assert_eq!(parts, vec!["echo hello world"]);
    }

    #[test]
    fn test_split_redirect_append() {
        // 基本的な >> リダイレクト
        let (cmd, redirect) = split_redirect("echo hello >> file.txt");
        assert_eq!(cmd, "echo hello");
        assert_eq!(redirect, Some((">>".to_string(), "file.txt".to_string())));
    }

    #[test]
    fn test_split_redirect_append_with_spaces() {
        // スペースありの >> リダイレクト
        let (cmd, redirect) = split_redirect("echo hello world  >>  output.log");
        assert_eq!(cmd, "echo hello world");
        assert_eq!(redirect, Some((">>".to_string(), "output.log".to_string())));
    }

    #[test]
    fn test_split_redirect_no_target() {
        // ターゲットなしの >> リダイレクト
        let (cmd, redirect) = split_redirect("echo hello >> ");
        assert_eq!(cmd, "echo hello >> ");
        assert_eq!(redirect, None);
    }

    #[test]
    fn test_find_redirect_position_append() {
        // >> が優先されることを確認
        assert_eq!(find_redirect_position("echo >> file"), Some((5, ">>")));
        assert_eq!(find_redirect_position("echo > file"), Some((5, ">")));
        assert_eq!(find_redirect_position("echo file"), None);
    }

    #[test]
    fn test_contains_redirect_append() {
        assert!(contains_redirect("echo >> file"));
        assert!(contains_redirect("echo > file"));
        assert!(!contains_redirect("echo file"));
    }

    #[test]
    fn test_split_redirect_input() {
        let (cmd, redirect) = split_redirect("cat < file.txt");
        assert_eq!(cmd, "cat");
        assert_eq!(redirect, Some(("<".to_string(), "file.txt".to_string())));
    }

    #[test]
    fn test_find_redirect_position_input() {
        assert_eq!(find_redirect_position("cat < file"), Some((4, "<")));
        // >> が優先されることを確認
        assert_eq!(find_redirect_position("cmd >> file"), Some((4, ">>")));
    }

    #[test]
    fn test_contains_redirect_input() {
        assert!(contains_redirect("cat < file"));
        assert!(contains_redirect("cmd > file"));
        assert!(contains_redirect("cmd >> file"));
    }

    #[test]
    fn test_contains_background() {
        assert!(contains_background("echo hello &"));
        assert!(contains_background("cat file.txt | grep pattern &"));
        assert!(!contains_background("echo hello"));
    }

    #[test]
    fn test_parse_heredoc_header_basic() {
        let result = parse_heredoc_header("cat <<EOF");
        assert_eq!(result, Some(("cat".to_string(), "EOF".to_string(), false)));
    }

    #[test]
    fn test_parse_heredoc_header_with_strip() {
        let result = parse_heredoc_header("cat <<-END");
        assert_eq!(result, Some(("cat".to_string(), "END".to_string(), true)));
    }

    #[test]
    fn test_parse_heredoc_header_with_spaces() {
        let result = parse_heredoc_header("  cat  <<  DELIMITER  ");
        assert_eq!(
            result,
            Some(("cat".to_string(), "DELIMITER".to_string(), false))
        );
    }

    #[test]
    fn test_parse_heredoc_header_no_delimiter() {
        let result = parse_heredoc_header("cat <<");
        assert_eq!(result, None);
    }

    #[test]
    fn test_contains_heredoc() {
        assert!(contains_heredoc("cat <<EOF"));
        assert!(contains_heredoc("cat <<-END"));
        assert!(!contains_heredoc("echo hello"));
        assert!(!contains_heredoc("cat <<< string")); // <<<は除外
    }

    #[test]
    fn test_split_by_semicolon_basic() {
        let parts = split_by_semicolon("echo a; echo b; echo c");
        assert_eq!(parts, vec!["echo a", "echo b", "echo c"]);
    }

    #[test]
    fn test_split_by_semicolon_empty() {
        let parts = split_by_semicolon("echo a;; echo b;");
        assert_eq!(parts, vec!["echo a", "echo b"]);
    }

    #[test]
    fn test_split_by_semicolon_whitespace() {
        let parts = split_by_semicolon("  echo a  ;  echo b  ;  ");
        assert_eq!(parts, vec!["echo a", "echo b"]);
    }

    #[test]
    fn test_split_by_semicolon_single() {
        let parts = split_by_semicolon("echo hello");
        assert_eq!(parts, vec!["echo hello"]);
    }
}
