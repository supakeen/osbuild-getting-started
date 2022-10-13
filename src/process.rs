use std::io;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::str;

pub fn run(cmd: &'static str, args: Vec<&'static str>) -> Output {
    let process = Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .expect("failed to exec");

    if !process.status.success() {
        io::stdout().write_all(&process.stdout).unwrap();
        io::stderr().write_all(&process.stderr).unwrap();
    }

    process
}

pub fn split_lines(output: Vec<u8>) -> Vec<String> {
    str::from_utf8(&output)
        .unwrap()
        .split(|c| c == '\n')
        .map(|l| l.to_string())
        .collect()
}
