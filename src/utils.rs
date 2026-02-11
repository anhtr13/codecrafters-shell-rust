use std::{
    fs::metadata,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, ExitStatus},
};

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

pub fn run_executable(path: &str, args: &Vec<String>) -> std::io::Result<ExitStatus> {
    Command::new(path).args(args).status()
}
