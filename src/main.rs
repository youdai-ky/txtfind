use std::env;
use std::fs;
use std::process;

#[derive(Debug, PartialEq)]
struct Config {
    pattern: String,
    file_name: String,
    ignore_case: bool,
    line_number: bool,
    count: bool,
}

#[derive(Debug, PartialEq)]
enum Command {
    Search(Config),
    Help,
    Version,
}

#[derive(Debug, PartialEq)]
struct MatchedLine {
    line_number: usize,
    text: String,
}

fn main() {
    let command = match parse_args(env::args().skip(1)) {
        Ok(command) => command,
        Err(message) => {
            eprintln!("{message}");
            eprintln!();
            eprintln!("{}", help_message());
            process::exit(1);
        }
    };

    match execute(command) {
        Ok(output) => print!("{output}"),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    }
}

fn execute(command: Command) -> Result<String, String> {
    match command {
        Command::Help => Ok(help_message()),
        Command::Version => Ok(format!("txtfind {}\n", env!("CARGO_PKG_VERSION"))),
        Command::Search(config) => run_search(&config),
    }
}

fn parse_args<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = String>,
{
    let mut pattern = None;
    let mut file_name = None;
    let mut ignore_case = false;
    let mut line_number = false;
    let mut count = false;

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-V" | "--version" => return Ok(Command::Version),
            "-i" | "--ignore-case" => ignore_case = true,
            "-n" | "--line-number" => line_number = true,
            "-c" | "--count" => count = true,
            _ if arg.starts_with('-') => {
                return Err(format!("unknown option: {arg}"));
            }
            _ => {
                if pattern.is_none() {
                    pattern = Some(arg);
                } else if file_name.is_none() {
                    file_name = Some(arg);
                } else {
                    return Err("too many arguments".to_string());
                }
            }
        }
    }

    let pattern = pattern.ok_or_else(|| "missing pattern".to_string())?;
    let file_name = file_name.ok_or_else(|| "missing file".to_string())?;

    Ok(Command::Search(Config {
        pattern,
        file_name,
        ignore_case,
        line_number,
        count,
    }))
}

fn run_search(config: &Config) -> Result<String, String> {
    let contents = fs::read_to_string(&config.file_name)
        .map_err(|error| format!("failed to read {}: {}", config.file_name, error))?;

    let matched_lines = find_matched_lines(&contents, &config.pattern, config.ignore_case);

    Ok(format_output(
        &matched_lines,
        config.line_number,
        config.count,
    ))
}

fn find_matched_lines(contents: &str, pattern: &str, ignore_case: bool) -> Vec<MatchedLine> {
    let search_pattern = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    contents
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let target_line = if ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            if target_line.contains(&search_pattern) {
                Some(MatchedLine {
                    line_number: index + 1,
                    text: line.to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn format_output(matched_lines: &[MatchedLine], line_number: bool, count: bool) -> String {
    if count {
        return format!("{}\n", matched_lines.len());
    }

    matched_lines
        .iter()
        .map(|matched_line| {
            if line_number {
                format!("{}: {}\n", matched_line.line_number, matched_line.text)
            } else {
                format!("{}\n", matched_line.text)
            }
        })
        .collect()
}

fn help_message() -> String {
    let message = r#"txtfind

テキストファイル内の文字列を検索するCLIツール

USAGE:
    txtfind <pattern> <file> [OPTIONS]

ARGS:
    <pattern>    検索する文字列
    <file>       検索対象のテキストファイル

OPTIONS:
    -i, --ignore-case     大文字・小文字を区別せずに検索する
    -n, --line-number     一致した行に行番号を付けて表示する
    -c, --count           一致件数のみを表示する
    -h, --help            ヘルプを表示する
    -V, --version         バージョン情報を表示する

EXAMPLES:
    txtfind "error" log.txt
    txtfind "TODO" memo.txt --line-number
    txtfind "error" log.txt --ignore-case
    txtfind "error" log.txt --count
"#;

    message.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_required_arguments() {
        let args = vec!["error".to_string(), "log.txt".to_string()];
        let command = parse_args(args).unwrap();

        assert_eq!(
            command,
            Command::Search(Config {
                pattern: "error".to_string(),
                file_name: "log.txt".to_string(),
                ignore_case: false,
                line_number: false,
                count: false,
            })
        );
    }

    #[test]
    fn parse_options() {
        let args = vec![
            "error".to_string(),
            "log.txt".to_string(),
            "--ignore-case".to_string(),
            "--line-number".to_string(),
            "--count".to_string(),
        ];
        let command = parse_args(args).unwrap();

        assert_eq!(
            command,
            Command::Search(Config {
                pattern: "error".to_string(),
                file_name: "log.txt".to_string(),
                ignore_case: true,
                line_number: true,
                count: true,
            })
        );
    }

    #[test]
    fn find_lines() {
        let contents = "error: first\ninfo: message\nerror: second\n";
        let matched_lines = find_matched_lines(contents, "error", false);

        assert_eq!(
            matched_lines,
            vec![
                MatchedLine {
                    line_number: 1,
                    text: "error: first".to_string(),
                },
                MatchedLine {
                    line_number: 3,
                    text: "error: second".to_string(),
                },
            ]
        );
    }

    #[test]
    fn find_lines_with_ignore_case() {
        let contents = "Error: first\ninfo: message\nERROR: second\n";
        let matched_lines = find_matched_lines(contents, "error", true);

        assert_eq!(matched_lines.len(), 2);
    }

    #[test]
    fn format_lines_without_line_numbers() {
        let matched_lines = vec![
            MatchedLine {
                line_number: 1,
                text: "error: first".to_string(),
            },
            MatchedLine {
                line_number: 3,
                text: "error: second".to_string(),
            },
        ];

        let output = format_output(&matched_lines, false, false);

        assert_eq!(output, "error: first\nerror: second\n");
    }

    #[test]
    fn format_lines_with_line_numbers() {
        let matched_lines = vec![MatchedLine {
            line_number: 3,
            text: "TODO: write README".to_string(),
        }];

        let output = format_output(&matched_lines, true, false);

        assert_eq!(output, "3: TODO: write README\n");
    }

    #[test]
    fn format_count() {
        let matched_lines = vec![
            MatchedLine {
                line_number: 1,
                text: "error: first".to_string(),
            },
            MatchedLine {
                line_number: 3,
                text: "error: second".to_string(),
            },
        ];

        let output = format_output(&matched_lines, false, true);

        assert_eq!(output, "2\n");
    }
}