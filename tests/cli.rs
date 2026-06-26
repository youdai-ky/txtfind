use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

fn prepare_test_directory(name: &str) -> std::path::PathBuf {
    let directory = std::env::temp_dir().join(format!("txtfind_{name}_{}", std::process::id()));

    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).unwrap();

    fs::write(directory.join("log1.txt"), "error: first\ninfo: message\n").unwrap();
    fs::write(directory.join("log2.txt"), "warning: message\nerror: second\n").unwrap();

    directory
}

#[test]
fn search_multiple_files_interactively() {
    let directory = prepare_test_directory("multi");

    let mut child = Command::new(env!("CARGO_BIN_EXE_txtfind"))
        .current_dir(&directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(b"1,2\nerror\n").unwrap();
    }

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("log1.txt:1: error: first"));
    assert!(stdout.contains("log2.txt:2: error: second"));

    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn count_matches_interactively() {
    let directory = prepare_test_directory("count");

    let mut child = Command::new(env!("CARGO_BIN_EXE_txtfind"))
        .arg("--count")
        .current_dir(&directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(b"1,2\nerror\n").unwrap();
    }

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.trim_end().ends_with("2"));

    let _ = fs::remove_dir_all(&directory);
}
