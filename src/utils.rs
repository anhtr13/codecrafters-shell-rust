use std::{fs::metadata, os::unix::fs::PermissionsExt, path::Path, process::Command};

use crate::CmdOutput;

pub fn parse_input(input: &str) -> Option<(String, Vec<String>)> {
    if let Some(mut cmd) = shlex::split(input) {
        let args = cmd.split_off(1);
        return Some((cmd.remove(0), args));
    }
    None
}

pub fn find_excutable(name: &str) -> Option<String> {
    let path = std::env::var("PATH").expect("cannot get PATH");
    let bins: Vec<&str> = path.split(':').collect();
    for bin in bins {
        let p = format!("{bin}/{name}");
        let path = Path::new(&p);
        if path.is_file() {
            let mode = metadata(path).unwrap().permissions().mode();
            if mode & 0o100 != 0 || mode & 0o010 != 0 || mode & 0o001 != 0 {
                return Some(format!("{bin}/{name}"));
            }
        }
    }
    None
}

pub fn run_executable(path: &str, args: &Vec<String>) -> CmdOutput {
    match Command::new(path).args(args).output() {
        Ok(output) => {
            let mut std_err = output.stderr;
            if let Some(c) = std_err.last()
                && *c == b'\n'
            {
                std_err.pop();
            }
            let std_err = String::from_utf8(std_err).unwrap();
            let mut std_out = output.stdout;
            if let Some(c) = std_out.last()
                && *c == b'\n'
            {
                std_out.pop();
            }
            let std_out = String::from_utf8(std_out).unwrap();
            let status = if std_err.is_empty() { 0 } else { 1 };
            CmdOutput {
                status,
                std_out,
                std_err,
            }
        }
        Err(e) => CmdOutput {
            status: 1,
            std_out: "".to_string(),
            std_err: e.to_string(),
        },
    }
}
