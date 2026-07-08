mod gencomp;

use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    name = "txtfind",
    version,
    about = "テキストファイル内の文字列を検索するCLIツール"
)]
pub struct Args {
    #[arg(
        short = 'i',
        long = "ignore-case",
        help = "大文字・小文字を区別せずに検索する"
    )]
    ignore_case: bool,

    #[arg(
        short = 'n',
        long = "line-number",
        help = "行番号を表示する（現在はデフォルトで表示）"
    )]
    line_number: bool,

    #[arg(short = 'c', long = "count", help = "一致件数のみを表示する")]
    count: bool,

    #[arg(long = "completions", help = "各シェル用の補完ファイルを生成する")]
    completions: bool,
}

#[derive(Debug, PartialEq)]
struct Config {
    ignore_case: bool,
    count: bool,
}

#[derive(Debug, PartialEq)]
struct MatchedLine {
    file_name: String,
    line_number: usize,
    text: String,
}

fn main() {
    let args = Args::parse();

    if args.completions {
        gencomp::generate(Path::new("completions"));
        println!("generated completion files in completions/");
        return;
    }

    let _line_number = args.line_number;

    let config = Config {
        ignore_case: args.ignore_case,
        count: args.count,
    };

    match run_interactive_search(&config) {
        Ok(output) => print!("{output}"),
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parse_no_options() {
        let args = Args::try_parse_from(["txtfind"]).unwrap();

        assert!(!args.ignore_case);
        assert!(!args.line_number);
        assert!(!args.count);
        assert!(!args.completions);
    }

    #[test]
    fn parse_ignore_case_option() {
        let args = Args::try_parse_from(["txtfind", "--ignore-case"]).unwrap();

        assert!(args.ignore_case);
        assert!(!args.count);
        assert!(!args.completions);
    }

    #[test]
    fn parse_count_option() {
        let args = Args::try_parse_from(["txtfind", "--count"]).unwrap();

        assert!(!args.ignore_case);
        assert!(args.count);
        assert!(!args.completions);
    }

    #[test]
    fn parse_line_number_option() {
        let args = Args::try_parse_from(["txtfind", "--line-number"]).unwrap();

        assert!(args.line_number);
    }

    #[test]
    fn parse_completions_option() {
        let args = Args::try_parse_from(["txtfind", "--completions"]).unwrap();

        assert!(args.completions);
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
    fn reject_empty_selection() {
        let result = parse_selected_indexes("", 5);

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

        assert_eq!(
            matched_lines,
            vec![
                MatchedLine {
                    file_name: "log.txt".to_string(),
                    line_number: 1,
                    text: "Error: first".to_string(),
                },
                MatchedLine {
                    file_name: "log.txt".to_string(),
                    line_number: 3,
                    text: "ERROR: second".to_string(),
                },
            ]
        );
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