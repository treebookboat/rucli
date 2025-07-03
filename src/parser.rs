use crate::commands::{Command, CommandInfo, COMMANDS};

// コマンド情報の取得
fn find_command(name: &str) -> Option<&CommandInfo>
{
    COMMANDS.iter().find(|command| command.name == name)
}

// 引数チェック
fn validate_args(cmd_info: &CommandInfo, args: &[&str]) -> Result<(),String>
{
    // 引数の個数
    let arg_count = args.len();

    // 最小引数チェック
    if arg_count < cmd_info.min_args {
        return Err([
            format!("Error: {} requires at least {} argument(s)", cmd_info.name, cmd_info.min_args),
            format!("Usage: {}", cmd_info.usage)
        ].join("\n"));
    }

    // 最大引数チェック
    if let Some(max) = cmd_info.max_args {
        if max < arg_count {
                    return Err([
            format!("Error: {} accepts at most {} argument(s)", cmd_info.name, max),
            format!("Usage: {}", cmd_info.usage)
        ].join("\n"));
        }
    }

    // ここまでくればOK
    Ok(())
}

// 文字列のパース
pub fn parse_command(input : &str) -> Result<Command, String>{

    let parts: Vec<&str> = input.split_whitespace().collect();

    // 空入力チェック
    if parts.is_empty() {
        return Err("No command provided".to_string());
    }

    let cmd_name = parts[0];
    let args = &parts[1..];

    // 引数の数チェック
    if let Some(cmd_info) = find_command(cmd_name)
    {
        validate_args(cmd_info, args)?;
    }

    match parts.as_slice() {
        ["help"] => Ok(Command::Help),
        ["echo", message @ ..] => Ok(Command::Echo{message : message.join(" ")}),
        ["cat" , filename] => Ok(Command::Cat { filename : filename.to_string() }),
        ["write", filename,content @ ..] => Ok(Command::Write { filename : filename.to_string(), content : content.join(" ")}),
        ["ls"] => Ok(Command::Ls),
        ["repeat", count , message @ ..] => {
            match count.parse::<i32>() {
                Ok(count) if count > 0 => Ok(Command::Repeat{count, message : message.join(" ") }),
                Ok(_) => Err("Error : count must be positive".to_string()),
                Err(_) => Err(format!("Error: {} isn't a valid number", count)),
            }
        },
        ["exit"] | ["quit"] => Ok(Command::Exit),
        commands => Err(format!("Unknown command: {}", commands.join(" "))),
    }
}