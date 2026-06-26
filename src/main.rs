/*
txtfind は，現在のディレクトリにあるファイルを一覧表示し，
ユーザが選択した複数ファイルから指定文字列を検索するCLIツールである。

処理を関数ごとに分けることで，入力処理，ファイル一覧取得，検索処理，
出力整形をそれぞれテストしやすい形にしている。
*/

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

#[derive(Debug, PartialEq)]
struct Config {
    ignore_case: bool,
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
    file_name: String,
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
        Command::Search(config) => run_interactive_search(&config),
    }
}

fn parse_args<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = String>,
{
    let mut ignore_case = false;
    let mut count = false;

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-V" | "--version" => return Ok(Command::Version),
            "-i" | "--ignore-case" => ignore_case = true,
            "-c" | "--count" => count = true,
            "-n" | "--line-number" => {
                // 行番号は現在デフォルトで表示するため，互換用に受け付ける。
            }
            _ => return Err(format!("unknown option: {arg}")),
        }
    }

    Ok(Command::Search(Config { ignore_case, count }))
}

fn run_interactive_search(config: &Config) -> Result<String, String> {
    let file_names = collect_current_directory_files()?;

    if file_names.is_empty() {
        return Err("current directory has no files".to_string());
    }

    print_file_list(&file_names);

    let selected_indexes = read_selected_indexes(file_names.len())?;
    let selected_files = select_file_names(&file_names, &selected_indexes);

    let pattern = read_pattern()?;

    let mut all_matched_lines = Vec::new();

    for file_name in &selected_files {
        let contents = fs::read_to_string(file_name)
            .map_err(|error| format!("failed to read {file_name}: {error}"))?;

        let matched_lines = find_matched_lines(file_name, &contents, &pattern, config.ignore_case);
        all_matched_lines.extend(matched_lines);
    }

    Ok(format_output(&all_matched_lines, config.count))
}

fn collect_current_directory_files() -> Result<Vec<String>, String> {
    let mut file_names = Vec::new();

    let entries = fs::read_dir(".").map_err(|error| format!("failed to read directory: {error}"))?;

    for entry in entries {
        let entry = entry.map_err(|error| format!("failed to read entry: {error}"))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                file_names.push(file_name.to_string());
            }
        }
    }

    file_names.sort();
    Ok(file_names)
}

fn print_file_list(file_names: &[String]) {
    println!("Files in current directory:");

    for (index, file_name) in file_names.iter().enumerate() {
        println!("  {}. {}", index + 1, file_name);
    }

    println!();
}

fn read_selected_indexes(file_count: usize) -> Result<Vec<usize>, String> {
    print!("Select file numbers: ");
    io::stdout()
        .flush()
        .map_err(|error| format!("failed to flush stdout: {error}"))?;

    let input = read_line()?;
    parse_selected_indexes(&input, file_count)
}

fn read_pattern() -> Result<String, String> {
    print!("Search pattern: ");
    io::stdout()
        .flush()
        .map_err(|error| format!("failed to flush stdout: {error}"))?;

    let pattern = read_line()?;

    if pattern.is_empty() {
        return Err("search pattern is empty".to_string());
    }

    Ok(pattern)
}

fn read_line() -> Result<String, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|error| format!("failed to read input: {error}"))?;

    Ok(input.trim().to_string())
}

fn parse_selected_indexes(input: &str, file_count: usize) -> Result<Vec<usize>, String> {
    let mut indexes = Vec::new();

    for value in input.split([',', ' ']) {
        if value.trim().is_empty() {
            continue;
        }

        let number = value
            .trim()
            .parse::<usize>()
            .map_err(|_| format!("invalid file number: {value}"))?;

        if number == 0 || number > file_count {
            return Err(format!("file number out of range: {number}"));
        }

        let index = number - 1;

        if !indexes.contains(&index) {
            indexes.push(index);
        }
    }

    if indexes.is_empty() {
        return Err("no file selected".to_string());
    }

    Ok(indexes)
}

fn select_file_names(file_names: &[String], indexes: &[usize]) -> Vec<String> {
    indexes
        .iter()
        .map(|index| file_names[*index].clone())
        .collect()
}

fn find_matched_lines(
    file_name: &str,
    contents: &str,
    pattern: &str,
    ignore_case: bool,
) -> Vec<MatchedLine> {
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
                    file_name: file_name.to_string(),
                    line_number: index + 1,
                    text: line.to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn format_output(matched_lines: &[MatchedLine], count: bool) -> String {
    if count {
        return format!("{}\n", matched_lines.len());
    }

    matched_lines.iter().map(format_matched_line).collect()
}

fn format_matched_line(matched_line: &MatchedLine) -> String {
    format!(
        "{}:{}: {}\n",
        matched_line.file_name, matched_line.line_number, matched_line.text
    )
}

fn help_message() -> String {
    let message = r#"txtfind

テキストファイル内の文字列を検索するCLIツール

USAGE:
    txtfind [OPTIONS]

OPTIONS:
    -i, --ignore-case     大文字・小文字を区別せずに検索する
    -n, --line-number     行番号を表示する（現在はデフォルトで表示）
    -c, --count           一致件数のみを表示する
    -h, --help            ヘルプを表示する
    -V, --version         バージョン情報を表示する

INTERACTIVE FLOW:
    1. 現在の階層にあるファイル一覧を表示する
    2. 検索対象ファイルの番号を入力する
    3. 検索文字列を入力する
    4. 検索結果を表示する

OUTPUT FORMAT:
    file_name:line_number: matched line

FILE SELECTION:
    1 2 3
    1,2,3
"#;

    message.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};

    #[test]
    fn parse_no_options() {
        let args: Vec<String> = vec![];
        let command = parse_args(args).unwrap();

        assert_eq!(
            command,
            Command::Search(Config {
                ignore_case: false,
                count: false,
            })
        );
    }

    #[test]
    fn parse_options() {
        let args = vec![
            "--ignore-case".to_string(),
            "--line-number".to_string(),
            "--count".to_string(),
        ];
        let command = parse_args(args).unwrap();

        assert_eq!(
            command,
            Command::Search(Config {
                ignore_case: true,
                count: true,
            })
        );
    }

    #[test]
    fn parse_selected_numbers_with_space() {
        let indexes = parse_selected_indexes("1 3", 5).unwrap();

        assert_eq!(indexes, vec![0, 2]);
    }

    #[test]
    fn parse_selected_numbers_with_comma() {
        let indexes = parse_selected_indexes("1,2,4", 5).unwrap();

        assert_eq!(indexes, vec![0, 1, 3]);
    }

    #[test]
    fn parse_selected_numbers_removes_duplicates() {
        let indexes = parse_selected_indexes("1,1,2", 5).unwrap();

        assert_eq!(indexes, vec![0, 1]);
    }

    #[test]
    fn reject_out_of_range_number() {
        let result = parse_selected_indexes("1,6", 5);

        assert!(result.is_err());
    }

    #[test]
    fn select_files() {
        let file_names = vec![
            "log1.txt".to_string(),
            "log2.txt".to_string(),
            "memo.txt".to_string(),
        ];
        let indexes = vec![0, 2];

        let selected_files = select_file_names(&file_names, &indexes);

        assert_eq!(
            selected_files,
            vec!["log1.txt".to_string(), "memo.txt".to_string()]
        );
    }

    #[test]
    fn collect_files_includes_hidden_files() {
        let original_dir = env::current_dir().unwrap();
        let test_dir = env::temp_dir().join(format!(
            "txtfind_unit_test_{}",
            std::process::id()
        ));

        let _ = fs::remove_dir_all(&test_dir);
        fs::create_dir_all(&test_dir).unwrap();

        File::create(test_dir.join("memo.txt")).unwrap();
        File::create(test_dir.join(".hidden.txt")).unwrap();
        fs::create_dir_all(test_dir.join("directory")).unwrap();

        env::set_current_dir(&test_dir).unwrap();
        let file_names = collect_current_directory_files().unwrap();
        env::set_current_dir(original_dir).unwrap();

        let _ = fs::remove_dir_all(&test_dir);

        assert_eq!(
            file_names,
            vec![".hidden.txt".to_string(), "memo.txt".to_string()]
        );
    }

    #[test]
    fn find_lines() {
        let contents = "error: first\ninfo: message\nerror: second\n";
        let matched_lines = find_matched_lines("log.txt", contents, "error", false);

        assert_eq!(
            matched_lines,
            vec![
                MatchedLine {
                    file_name: "log.txt".to_string(),
                    line_number: 1,
                    text: "error: first".to_string(),
                },
                MatchedLine {
                    file_name: "log.txt".to_string(),
                    line_number: 3,
                    text: "error: second".to_string(),
                },
            ]
        );
    }

    #[test]
    fn find_lines_with_ignore_case() {
        let contents = "Error: first\ninfo: message\nERROR: second\n";
        let matched_lines = find_matched_lines("log.txt", contents, "error", true);

        assert_eq!(matched_lines.len(), 2);
    }

    #[test]
    fn format_lines() {
        let matched_lines = vec![
            MatchedLine {
                file_name: "log1.txt".to_string(),
                line_number: 1,
                text: "error: first".to_string(),
            },
            MatchedLine {
                file_name: "log2.txt".to_string(),
                line_number: 2,
                text: "error: second".to_string(),
            },
        ];

        let output = format_output(&matched_lines, false);

        assert_eq!(
            output,
            "log1.txt:1: error: first\nlog2.txt:2: error: second\n"
        );
    }

    #[test]
    fn format_count() {
        let matched_lines = vec![
            MatchedLine {
                file_name: "log1.txt".to_string(),
                line_number: 1,
                text: "error: first".to_string(),
            },
            MatchedLine {
                file_name: "log2.txt".to_string(),
                line_number: 3,
                text: "error: second".to_string(),
            },
        ];

        let output = format_output(&matched_lines, true);

        assert_eq!(output, "2\n");
    }
}
