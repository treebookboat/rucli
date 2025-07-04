use crate::handles::*;

// 実行できるコマンド群
pub enum Command {
    Help,
    Echo{message : String},
    Repeat{count : i32, message : String},
    Cat{filename : String},
    Write{filename : String, content : String},
    Ls,
    Exit,
}

pub struct CommandInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub min_args: usize,
    pub max_args: Option<usize>,
}

pub const COMMANDS: &[CommandInfo] = &[
    CommandInfo {
        name: "help",
        description: "Show this help message",
        usage: "help",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "echo",
        description: "Display message",
        usage: "echo <message...>",
        min_args: 1,
        max_args: None,
    },
    CommandInfo {
        name: "cat",
        description: "Display file contents",
        usage: "cat <filename>",
        min_args: 1,
        max_args: Some(1),
    },
    CommandInfo {
        name: "write",
        description: "Write content to file",
        usage: "write <filename> <content...>",
        min_args: 2,
        max_args: None,
    },
    CommandInfo {
        name: "ls",
        description: "List directory contents",
        usage: "ls",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "repeat",
        description: "Repeat message count times",
        usage: "repeat <count> <message...>",
        min_args: 2,
        max_args: None,
    },
    CommandInfo {
        name: "exit",
        description: "Exit the program",
        usage: "exit",
        min_args: 0,
        max_args: Some(0),
    },
    CommandInfo {
        name: "quit",
        description: "Exit the program",
        usage: "quit",
        min_args: 0,
        max_args: Some(0),
    },
];

// 命令の実行
pub fn execute_command(command : Command)
{
    match command {
        Command::Help => handle_help(),
        Command::Cat { filename } => handle_cat(&filename),
        Command::Echo{message} => println!("{}", message),
        Command::Write { filename, content } => handle_write(&filename, &content),
        Command::Repeat{count, message} => handle_repeat(count, &message),
        Command::Ls => handle_ls(),
        Command::Exit => handle_exit(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //被りがないかチェック
    fn test_command_info_no_duplicates() {
        let mut names: Vec<&str> = COMMANDS.iter().map(|c| c.name).collect();
        names.sort();

        for i in 1..names.len() {
            assert_ne!(names[i], names[i-1], "Duplicate {}", names[i]);
        }
    }

    #[test]
    // min/maxの論理エラーを検出
    fn test_command_info_valid_args() {
        for cmd in COMMANDS
        {
            if let Some(max) = cmd.max_args
            {
                assert!(cmd.min_args <= max)
            }
        }
    }
}