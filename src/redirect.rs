//! リダイレクト処理を提供するモジュール

use log::debug;

use crate::commands::{Command, execute_command_get_output};
use crate::error::{Result, RucliError};
use std::fs::{self, OpenOptions};
use std::io::Write;

/// リダイレクトを実行
pub fn execute_redirect(command: Command, redirect_type: &str, target: &str) -> Result<String> {
    match redirect_type {
        ">" => {
            // コマンドからの出力を取得
            let output = execute_command_get_output(command, None)?;

            // ファイルに書き込み
            fs::write(target, output)?;

            Ok(String::new())
        }
        ">>" => {
            // コマンドからの出力を取得
            let output = execute_command_get_output(command, None)?;

            // 追記モードでファイルを開く
            let mut file = OpenOptions::new().append(true).create(true).open(target)?;

            // 書き込み
            write!(file, "{output}")?;

            Ok(String::new())
        }
        "<" => {
            debug!("Input redirect from file: '{target}'");

            // ファイルの内容を読み込む
            let input_content = fs::read_to_string(target)?;

            // コマンドを入力付きで実行
            let output = execute_command_get_output(command, Some(&input_content))?;

            // 結果を出力
            // if !output.is_empty() {
            //     println!("{output}");
            // }

            Ok(output)
        }
        _ => Err(RucliError::ParseError(
            "undefined redirect command".to_string(),
        )),
    }
}
