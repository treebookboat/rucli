//! リダイレクト処理を提供するモジュール

use crate::commands::{Command, execute_command_get_output};
use crate::error::{Result, RucliError};
use std::fs;

/// リダイレクトを実行
pub fn execute_redirect(command: Command, redirect_type: &str, target: &str) -> Result<()> {
    match redirect_type {
        ">" => {
            // コマンドからの出力を取得
            let output = execute_command_get_output(command, None)?;

            // ファイルに書き込み
            fs::write(target, output)?;

            Ok(())
        }
        ">>" => {
            // PR #50で実装
            todo!("Append redirect not implemented yet")
        }
        "<" => {
            // PR #51で実装
            todo!("Input redirect not implemented yet")
        }
        _ => Err(RucliError::ParseError(
            "undefined redirect command".to_string(),
        )),
    }
}
