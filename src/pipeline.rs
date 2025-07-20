//! パイプラインに関連する関数を提供するモジュール

use crate::{commands::execute_command_internal, error::Result, parser::parse_command};

/// パイプラインで繋がれた複数のコマンドを表現
pub struct PipelineCommand {
    commands: Vec<String>, // 例: ["echo hello", "grep h", "wc -l"]
}

impl PipelineCommand {
    // コンストラクタ
    pub fn new(commands: Vec<String>) -> Self {
        PipelineCommand { commands }
    }

    // コマンド群取得
    pub fn commands(&self) -> &[String] {
        &self.commands
    }
}

/// パイプラインを実行する構造体
pub struct PipelineExecutor;

impl PipelineExecutor {
    // 文字列として結果を返す
    pub fn execute(pipeline: &PipelineCommand) -> Result<String> {
        let commands = pipeline.commands();

        if commands.is_empty() {
            return Ok(String::new());
        }

        let mut previous_output = String::new();

        for (i, cmd_str) in commands.iter().enumerate() {
            let cmd = parse_command(cmd_str)?;
            let input = if i == 0 {
                None
            } else {
                Some(previous_output.as_str())
            };
            previous_output = execute_command_internal(cmd, input)?;
        }

        Ok(previous_output)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::split_by_pipe;

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
}
