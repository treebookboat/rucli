//! ファイル操作コマンドのパース関数

use crate::commands::Command;
use crate::error::Result;

pub(super) fn parse_mkdir(args: &[&str]) -> Result<Command> {
    match args {
        ["-p", path] => Ok(Command::Mkdir {
            path: path.to_string(),
            parents: true,
        }),
        [path] => Ok(Command::Mkdir {
            path: path.to_string(),
            parents: false,
        }),
        _ => unreachable!(),
    }
}

pub(super) fn parse_rm(args: &[&str]) -> Result<Command> {
    match args {
        ["-r", path] => Ok(Command::Rm {
            path: path.to_string(),
            recursive: true,
            force: false,
        }),
        ["-f", path] => Ok(Command::Rm {
            path: path.to_string(),
            recursive: false,
            force: true,
        }),
        ["-rf", path] | ["-fr", path] => Ok(Command::Rm {
            path: path.to_string(),
            recursive: true,
            force: true,
        }),
        [path] => Ok(Command::Rm {
            path: path.to_string(),
            recursive: false,
            force: false,
        }),
        _ => unreachable!(),
    }
}

pub(super) fn parse_cp(args: &[&str]) -> Result<Command> {
    match args {
        ["-r", src, dst] => Ok(Command::Cp {
            source: src.to_string(),
            destination: dst.to_string(),
            recursive: true,
        }),
        [src, dst] => Ok(Command::Cp {
            source: src.to_string(),
            destination: dst.to_string(),
            recursive: false,
        }),
        _ => unreachable!(),
    }
}

pub(super) fn parse_mv(args: &[&str]) -> Result<Command> {
    Ok(Command::Mv {
        source: args[0].to_string(),
        destination: args[1].to_string(),
    })
}

pub(super) fn parse_find(args: &[&str]) -> Result<Command> {
    match args.len() {
        1 => Ok(Command::Find {
            path: None,
            name: args[0].to_string(),
        }),
        2 => Ok(Command::Find {
            path: Some(args[0].to_string()),
            name: args[1].to_string(),
        }),
        _ => unreachable!(),
    }
}

pub(super) fn parse_grep(args: &[&str]) -> Result<Command> {
    Ok(Command::Grep {
        pattern: args[0].to_string(),
        files: args[1..].iter().map(|f| f.to_string()).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mkdir_simple() {
        let result = parse_mkdir(&["testdir"]);
        assert!(matches!(result, Ok(Command::Mkdir { path, parents: false }) if path == "testdir"));
    }

    #[test]
    fn test_parse_mkdir_with_parents() {
        let result = parse_mkdir(&["-p", "path/to/dir"]);
        assert!(
            matches!(result, Ok(Command::Mkdir { path, parents: true }) if path == "path/to/dir")
        );
    }

    #[test]
    fn test_parse_rm_simple() {
        let result = parse_rm(&["file.txt"]);
        assert!(
            matches!(result, Ok(Command::Rm { path, recursive: false, force: false }) if path == "file.txt")
        );
    }

    #[test]
    fn test_parse_rm_recursive() {
        let result = parse_rm(&["-r", "dir"]);
        assert!(
            matches!(result, Ok(Command::Rm { path, recursive: true, force: false }) if path == "dir")
        );
    }

    #[test]
    fn test_parse_rm_force() {
        let result = parse_rm(&["-f", "file"]);
        assert!(
            matches!(result, Ok(Command::Rm { path, recursive: false, force: true }) if path == "file")
        );
    }

    #[test]
    fn test_parse_rm_recursive_force() {
        let result = parse_rm(&["-rf", "dir"]);
        assert!(
            matches!(result, Ok(Command::Rm { path, recursive: true, force: true }) if path == "dir")
        );

        let result2 = parse_rm(&["-fr", "dir"]);
        assert!(
            matches!(result2, Ok(Command::Rm { path, recursive: true, force: true }) if path == "dir")
        );
    }

    #[test]
    fn test_parse_cp_simple() {
        let result = parse_cp(&["src.txt", "dst.txt"]);
        match result {
            Ok(Command::Cp {
                source,
                destination,
                recursive: false,
            }) => {
                assert_eq!(source, "src.txt");
                assert_eq!(destination, "dst.txt");
            }
            _ => panic!("Expected Cp command"),
        }
    }

    #[test]
    fn test_parse_cp_recursive() {
        let result = parse_cp(&["-r", "srcdir", "dstdir"]);
        match result {
            Ok(Command::Cp {
                source,
                destination,
                recursive: true,
            }) => {
                assert_eq!(source, "srcdir");
                assert_eq!(destination, "dstdir");
            }
            _ => panic!("Expected recursive Cp command"),
        }
    }

    #[test]
    fn test_parse_mv() {
        let result = parse_mv(&["old.txt", "new.txt"]);
        match result {
            Ok(Command::Mv {
                source,
                destination,
            }) => {
                assert_eq!(source, "old.txt");
                assert_eq!(destination, "new.txt");
            }
            _ => panic!("Expected Mv command"),
        }
    }

    #[test]
    fn test_parse_find_current_dir() {
        let result = parse_find(&["*.txt"]);
        assert!(matches!(result, Ok(Command::Find { path: None, name }) if name == "*.txt"));
    }

    #[test]
    fn test_parse_find_with_path() {
        let result = parse_find(&["/home", "*.log"]);
        match result {
            Ok(Command::Find { path, name }) => {
                assert_eq!(path, Some("/home".to_string()));
                assert_eq!(name, "*.log");
            }
            _ => panic!("Expected Find command"),
        }
    }

    #[test]
    fn test_parse_grep_single_file() {
        let result = parse_grep(&["pattern", "file.txt"]);
        match result {
            Ok(Command::Grep { pattern, files }) => {
                assert_eq!(pattern, "pattern");
                assert_eq!(files, vec!["file.txt"]);
            }
            _ => panic!("Expected Grep command"),
        }
    }

    #[test]
    fn test_parse_grep_multiple_files() {
        let result = parse_grep(&["error", "log1.txt", "log2.txt", "log3.txt"]);
        match result {
            Ok(Command::Grep { pattern, files }) => {
                assert_eq!(pattern, "error");
                assert_eq!(files, vec!["log1.txt", "log2.txt", "log3.txt"]);
            }
            _ => panic!("Expected Grep command"),
        }
    }
}
