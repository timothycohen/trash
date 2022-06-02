use clap::Parser;
use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    Put,
    Restore,
    Empty,
}

impl FromStr for Method {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Method::*;

        match s {
            "p" | "put" | "Put" => Ok(Put),
            "r" | "restore" | "Restore" => Ok(Restore),
            "e" | "empty" | "Empty" => Ok(Empty),
            _ => Err("Valid methods are `put` `restore` `empty`"),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None, author)]
pub struct Args {
    /// `p` | `put`     `r` | `restore`     `e` | `empty`
    pub method: Method,

    /// The target file or directory
    pub file: String,

    /// Explain all steps
    #[clap(short, long)]
    pub verbose: bool,
}
