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

// todo. valid operations are put FILE; restore FILE; empty;
// but the usage details make that a bit unclear. It's probably better to make separate commands rather than overloading the method with a sometimes required argument

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None, author)]
pub struct Args {
    /// `p` | `put`     `r` | `restore`     `e` | `empty`
    pub method: Method,

    /// The target file or directory used with the `put` or `restore` methods
    pub file: Option<String>,

    /// Explain all steps
    #[clap(short, long)]
    pub verbose: bool,

    /// Force non-recoverable deletes/overwrites
    #[clap(short, long)]
    pub force: bool,
}
