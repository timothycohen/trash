use crate::args::{Args, Method};
use ansi_term::Colour;
use std::{env, ffi::OsString, fs, path::PathBuf};
use uuid::Uuid;

#[derive(Debug)]
pub struct Config {
    pub is_dir: bool,
    pub path: PathBuf,
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
                let path = match fs::canonicalize(&args.file) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("{} Unable to canonicalize file path: {}", Colour::Red.paint("Err:"), e);
                        eprintln!("{} Problem path: {}", Colour::Yellow.paint("Hint:"), args.file);
                        std::process::exit(1)
                    }
                };

                let file_basename = match path.file_name() {
                    Some(f) => f.to_owned(),
                    None => {
                        // the canonicalization should make this unreachable
                        eprintln!("{} Unable to parse file path: {:?}", Colour::Red.paint("Err:"), path);
                        std::process::exit(1)
                    }
                };

                let is_dir = path.is_dir();

                Config {
                    file_basename,
                    path,
                    is_dir,
                    uuid: Some(Uuid::new_v4()),
                }
            }

            // todo This doesn't canonicalize paths like /Users/tco/../tco/dev, so it might not find the path in the Trash/info files
            Method::Restore => {
                let pwd = match env::current_dir() {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("{} Unable to process pwd: {}", Colour::Red.paint("Err:"), e);
                        std::process::exit(1)
                    }
                };

                let path: PathBuf = [pwd, PathBuf::from(&args.file)].iter().collect();

                let file_basename = match path.file_name() {
                    Some(f) => f.to_owned(),
                    None => {
                        eprintln!("{} Unable to parse file path: {:?}", Colour::Red.paint("Err:"), path);
                        std::process::exit(1)
                    }
                };

                let is_dir = path.is_dir();

                Config {
                    file_basename,
                    path,
                    is_dir,
                    uuid: None,
                }
            }
        }
    }
}
