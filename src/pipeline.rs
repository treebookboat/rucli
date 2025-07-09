//! パイプラインに関連する関数を提供するモジュール

use crate::{commands::execute_command_get_output, error::Result, parser::parse_command};

/// パイプラインで繋がれた複数のコマンドを表現
pub struct PipelineCommand {
    commands: Vec<String>, // 例: ["echo hello", "grep h", "wc -l"]
}

impl PipelineCommand {
    // コンストラクタ
    pub fn new(commands: Vec<String>) -> Self {
        PipelineCommand { commands }
    }

    // コマンドの数を返す
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    // 空かどうか
    pub fn is_empty(&self) -> bool {
        self.commands.len() == 0
    }

    // コマンド群取得
    pub fn commands(&self) -> &[String] {
        &self.commands
    }
}

/// パイプラインを実行する構造体
pub struct PipelineExecutor;

impl PipelineExecutor {
    pub fn execute(pipeline: &PipelineCommand) -> Result<()> {
        let commands = pipeline.commands();

        if commands.is_empty() {
            return Ok(());
        }

        let mut previous_output = String::new();

        for (i, cmd_str) in commands.iter().enumerate() {
            let cmd = parse_command(cmd_str)?;
            let input = if i == 0 {
                None
            } else {
                Some(previous_output.as_str())
            };
            previous_output = execute_command_get_output(cmd, input)?;
        }
        if !previous_output.is_empty() {
            println!("{previous_output}");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::split_by_pipe;

    use super::*;

    #[test]
    fn test_pipeline_command_creation() {
        let commands = vec!["echo hello".to_string(), "grep h".to_string()];
        let pipeline = PipelineCommand::new(commands);
        assert_eq!(pipeline.len(), 2);
        assert!(!pipeline.is_empty());
    }

    #[test]
    fn test_empty_pipeline() {
        let pipeline = PipelineCommand::new(vec![]);
        assert_eq!(pipeline.len(), 0);
        assert!(pipeline.is_empty());
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
}
