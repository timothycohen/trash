use crate::args::{Args, Method};
use ansi_term::Colour;
use std::{
    env, fs,
    path::{Component, Path, PathBuf},
};

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
                let source_path = normalize_path(&source_path);

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

pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
