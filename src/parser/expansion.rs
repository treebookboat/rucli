//! 各種展開機能（履歴展開、変数展開、コマンド置換）

use crate::error::{Result, RucliError};
use crate::history::{
    get_history_by_number, get_history_by_offset, get_last_command, search_history_by_prefix,
};

/// 履歴展開を含むかチェック
///
/// # Arguments
/// * `input` - チェックする文字列
///
/// # Returns
/// * 履歴展開パターンが含まれる場合はtrue
pub fn contains_history_expansion(input: &str) -> bool {
    input.split_whitespace().any(|word| {
        word == "!!" ||                                    // !!
        (word.starts_with('!') && word[1..].parse::<i32>().is_ok()) ||  // !n or !-n
        (word.starts_with('!') && word.len() > 1 && word.chars().nth(1).is_some_and(|c| c.is_alphabetic())) // !string
    })
}

/// 履歴展開を実行
///
/// # Arguments
/// * `input` - 展開する文字列
///
/// # Returns
/// * 展開後の文字列
pub fn expand_history(input: &str) -> Result<String> {
    let expand_str = input
        .split_whitespace()
        .map(|word| {
            if word.starts_with('!') && word.len() > 1 {
                expand_history_pattern(word)
            } else {
                Ok(word.to_string())
            }
        })
        .collect::<Result<Vec<_>>>()?
        .join(" ");

    Ok(expand_str)
}

/// 単一の履歴展開パターンを処理
fn expand_history_pattern(pattern: &str) -> Result<String> {
    // 共通のエラー
    let error = || RucliError::InvalidArgument(format!("bash: {pattern}: event not found"));

    // /!で始まらない場合は早期リターン
    if !pattern.starts_with("!") || pattern.len() <= 1 {
        return Err(error());
    }

    let rest = &pattern[1..];

    let result = match rest.chars().next() {
        Some('!') => get_last_command(),
        Some('-') => rest[1..]
            .parse::<i32>()
            .ok()
            .and_then(|n| get_history_by_offset(-n)),
        Some(c) if c.is_numeric() => rest.parse::<usize>().ok().and_then(get_history_by_number),
        Some(c) if c.is_alphabetic() => search_history_by_prefix(rest),
        _ => None,
    };

    result.ok_or_else(error)
}
