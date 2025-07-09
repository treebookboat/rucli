//! パイプラインに関連する関数を提供するモジュール

use crate::error::Result;

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
}

/// パイプラインを実行する構造体
pub struct PipelineExecutor;

impl PipelineExecutor {
    pub fn execute(_pipeline: &PipelineCommand) -> Result<()> {
        todo!("Pipeline execution not implemented yet");
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
