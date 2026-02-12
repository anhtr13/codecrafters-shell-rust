use std::{
    env::{current_dir, home_dir, set_current_dir},
    error::Error,
    fmt::Display,
    path::Path,
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

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cd => write!(f, "cd"),
            Self::Echo => write!(f, "exit"),
            Self::Exit => write!(f, "exit"),
            Self::Pwd => write!(f, "exit"),
            Self::Type => write!(f, "exit"),
        }
    }
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

pub fn run_echo(args: &[String]) -> Result<String, Box<dyn Error>> {
    Ok(args.join(" "))
}

pub fn run_type(args: &[String]) -> Result<String, Box<dyn Error>> {
    if Builtin::from_str(&args[0]).is_ok() {
        Ok(format!("{} is a shell builtin", args[0]))
    } else if let Some(path) = crate::utils::find_excutable(&args[0]) {
        Ok(format!("{} is {path}", args[0]))
    } else {
        Ok(format!("{}: not found", args[0]))
    }
}

pub fn run_pwd() -> Result<String, Box<dyn Error>> {
    let path = current_dir()?;
    Ok(format!("{}", path.display()))
}

pub fn run_cd(args: &[String]) -> Result<String, Box<dyn Error>> {
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
    let _ = set_current_dir(path)?;
    Ok("".to_string())
}
