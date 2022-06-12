use crate::args::{Args, Method};
use ansi_term::Colour;
use std::{env, fs, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub source_path: PathBuf,
    pub file_basename: PathBuf,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        if args.verbose {
            println!("{} Canonicalizing file paths.", Colour::Blue.paint("Info:"));
        }

        match args.method {
            Method::Empty => unreachable!(),
            Method::Info => unreachable!(),
            Method::Put => {
                let user_path = path_arg_guard(args.file);
                let source_path = match fs::canonicalize(&user_path) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("{} Unable to canonicalize file path: {}", Colour::Red.paint("Err:"), e);
                        eprintln!("{} Problem path: {}", Colour::Yellow.paint("Hint:"), &user_path);
                        std::process::exit(1)
                    }
                };

                let file_basename: PathBuf = match source_path.file_name() {
                    Some(f) => f.into(),
                    None => {
                        // the canonicalization should make this unreachable
                        eprintln!(
                            "{} Unable to parse file source_path: {:?}",
                            Colour::Red.paint("Err:"),
                            source_path
                        );
                        std::process::exit(1)
                    }
                };

                Config {
                    file_basename,
                    source_path,
                }
            }

            // todo This doesn't canonicalize paths like /Users/tco/../tco/dev, so it might not find the path in the Trash/info files
            Method::Restore => {
                // todo. if no path, do an interactive restoration by showing the trash contents
                let user_path = path_arg_guard(args.file);

                let pwd = match env::current_dir() {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("{} Unable to process pwd: {}", Colour::Red.paint("Err:"), e);
                        std::process::exit(1)
                    }
                };

                let source_path: PathBuf = [pwd, PathBuf::from(user_path)].iter().collect();

                let file_basename: PathBuf = match source_path.file_name() {
                    Some(f) => f.into(),
                    None => {
                        eprintln!(
                            "{} Unable to parse file path: {:?}",
                            Colour::Red.paint("Err:"),
                            source_path
                        );
                        std::process::exit(1)
                    }
                };

                Config {
                    file_basename,
                    source_path,
                }
            }
        }
    }
}

pub fn path_arg_guard(user_path: Option<String>) -> String {
    if user_path.is_none() {
        eprintln!("{} The following arguments are required:", Colour::Red.paint("error:"));
        eprintln!("    {}\n", Colour::Green.paint("[FILE]"));
        eprintln!("For more information try {}", Colour::Green.paint("--help"));
        std::process::exit(1)
    }
    user_path.unwrap()
}
