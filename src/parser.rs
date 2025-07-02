use crate::commands::Command;

// 文字列のパース
pub fn parse_command(input : &str) -> Result<Command, String>{
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.as_slice() {
        ["help"] => Ok(Command::Help),
        ["echo"] => Err("Error : echo requires a message".to_string()),
        ["echo", message @ ..] => Ok(Command::Echo{message : message.join(" ")}),
        ["cat" , filename] => Ok(Command::Cat { filename : filename.to_string() }),
        ["cat"] => Err("Error: cat requires a filename".to_string()),
        ["write"] => Err("Error : write requires a filename and a content".to_string()),
        ["write", _] => Err("Error : write requires a content".to_string()),
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