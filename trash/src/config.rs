use crate::args::{Args, Method};
use ansi_term::Colour;
use std::{env, ffi::OsString, fs, path::PathBuf};
use uuid::Uuid;

#[derive(Debug)]
pub struct Config {
    pub is_dir: bool,
    pub source_path: PathBuf,
    pub file_basename: OsString,
    pub uuid: Option<Uuid>,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        if args.verbose {
            println!("{} Canonicalizing file paths.", Colour::Blue.paint("Info:"));
        }

        match args.method {
            Method::Empty => unreachable!(),
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

                let file_basename = match source_path.file_name() {
                    Some(f) => f.to_owned(),
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

                let is_dir = source_path.is_dir();

                Config {
                    file_basename,
                    source_path,
                    is_dir,
                    uuid: Some(Uuid::new_v4()),
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

                let file_basename = match source_path.file_name() {
                    Some(f) => f.to_owned(),
                    None => {
                        eprintln!(
                            "{} Unable to parse file path: {:?}",
                            Colour::Red.paint("Err:"),
                            source_path
                        );
                        std::process::exit(1)
                    }
                };

                let is_dir = source_path.is_dir();

                Config {
                    file_basename,
                    source_path,
                    is_dir,
                    uuid: None,
                }
            }
        }
    }
}

fn path_arg_guard(user_path: Option<String>) -> String {
    if user_path.is_none() {
        eprintln!("{} The following arguments are required:", Colour::Red.paint("error:"));
        eprintln!("    {}\n", Colour::Green.paint("[FILE]"));
        eprintln!("For more information try {}", Colour::Green.paint("--help"));
        std::process::exit(1)
    }
    user_path.unwrap()
}
