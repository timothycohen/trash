use clap::Parser;
use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    Put,
    Restore,
    Empty,
    Info,
}

impl FromStr for Method {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Method::*;

        match s {
            "p" | "put" | "Put" => Ok(Put),
            "r" | "restore" | "Restore" => Ok(Restore),
            "e" | "empty" | "Empty" => Ok(Empty),
            "i" | "info" => Ok(Info),
            _ => Err("Valid methods are `empty` 'info' `put` `restore`"),
        }
    }
}

// todo the usage details make method + option unclear.
// It's probably better to make separate commands rather than overloading the method with a sometimes required argument

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None, author)]
pub struct Args {
    /// `e` | `empty`   `i` | 'info'   `p` | `put`   `r` | `restore`
    pub method: Method,

    /// The target files or directories
    pub files: Vec<String>,

    /// Explain all steps
    #[clap(short, long)]
    pub verbose: bool,

    /// Force non-recoverable deletes/overwrites
    #[clap(short, long)]
    pub force: bool,

    /// All (wildcard `*` like matches)
    #[clap(short, long)]
    pub all: bool,
}
