use std::{
    env::{current_dir, home_dir, set_current_dir},
    fs::metadata,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, ExitStatus, exit},
    str::FromStr,
};

#[derive(Debug)]
pub enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

impl FromStr for Builtin {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Self::Exit),
            "echo" => Ok(Self::Echo),
            "type" => Ok(Self::Type),
            "pwd" => Ok(Self::Pwd),
            "cd" => Ok(Self::Cd),
            _ => Err("Not a builtin command"),
        }
    }
}

fn run_echo(args: &Vec<String>) {
    let output = args.join(" ");
    println!("{output}");
}

fn run_type(args: &Vec<String>) {
    if Builtin::from_str(&args[0]).is_ok() {
        println!("{} is a shell builtin", args[0]);
    } else if let Some(path) = find_excutable(&args[0]) {
        println!("{} is {path}", args[0])
    } else {
        println!("{}: not found", args[0]);
    }
}

fn run_pwd() {
    let path = current_dir().expect("Cannot get current directory.");
    println!("{}", path.display());
}

fn run_cd(args: &Vec<String>) {
    let path_string = if args.is_empty() {
        let home = home_dir().expect("Impossible to get home dir");
        home.display().to_string()
    } else {
        let mut input = args[0].to_string();
        if input.as_bytes().first() == Some(&b'~') {
            let home = home_dir().expect("Impossible to get home dir");
            input = format!("{}{}", home.display(), &input[1..]);
        }
        input
    };
    let path = Path::new(&path_string);
    if path.is_dir() {
        set_current_dir(path).unwrap_or_else(|_| {
            panic!("{}: No such file or directory", &path_string);
        })
    } else {
        eprintln!("{}: No such file or directory", &path_string);
    }
}

fn parse_input(input: &str) -> Option<(&str, Vec<String>)> {
    if let Some((cmd, args)) = input.split_once(char::is_whitespace)
        && let Some(args) = shlex::split(args.trim())
    {
        return Some((cmd, args));
    }
    None
}

fn find_excutable(name: &str) -> Option<String> {
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

fn run_executable(path: &str, args: &Vec<String>) -> io::Result<ExitStatus> {
    Command::new(path).args(args).status()
}

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();

    loop {
        match io::stdin().read_line(&mut buffer) {
            Err(e) => {
                eprintln!("Error when reading input: {e}");
                exit(1);
            }
            Ok(_) => {
                let input = buffer.trim();
                let (cmd, args) = parse_input(input).unwrap();

                match cmd {
                    "exit" => {
                        break;
                    }
                    "echo" => {
                        run_echo(&args);
                    }
                    "type" => {
                        run_type(&args);
                    }
                    "pwd" => {
                        run_pwd();
                    }
                    "cd" => {
                        run_cd(&args);
                    }
                    _ => {
                        if find_excutable(&cmd).is_some() {
                            let _ = run_executable(cmd, &args);
                        } else {
                            eprintln!("{}: command not found", cmd);
                        }
                    }
                }

                buffer.clear();

                print!("$ ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
