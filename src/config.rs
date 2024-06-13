use crate::args::Method;
use crate::GLOBAL;
use colored::Colorize;
use std::{
    env, fs,
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
pub struct Config {
    pub source_path: PathBuf,
    pub file_basename: PathBuf,
}

impl Config {
    pub fn new(method: &Method, user_path: String) -> Self {
        if GLOBAL.verbose() {
            println!("{} Canonicalizing file paths.", "Info:".blue());
        }

        match method {
            Method::Empty => unreachable!(),
            Method::Info => unreachable!(),
            Method::Put => {
                let source_path = match fs::canonicalize(&user_path) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("{} Unable to canonicalize file path: {}", "Err:".red(), e);
                        eprintln!("{} Does this problem path exist? {}", "Hint:".yellow(), &user_path);
                        std::process::exit(1)
                    }
                };

                let file_basename: PathBuf = match source_path.file_name() {
                    Some(f) => f.into(),
                    None => {
                        // the canonicalization should make this unreachable
                        eprintln!("{} Unable to parse file source path: {:?}", "Err:".red(), source_path);
                        std::process::exit(1)
                    }
                };

                Config {
                    file_basename,
                    source_path,
                }
            }

            Method::Restore => {
                let pwd = match env::current_dir() {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("{} Unable to process pwd: {}", "Err:".red(), e);
                        std::process::exit(1)
                    }
                };

                let source_path: PathBuf = [pwd, PathBuf::from(user_path)].iter().collect();
                let source_path = normalize_path(&source_path);

                let file_basename: PathBuf = match source_path.file_name() {
                    Some(f) => f.into(),
                    None => {
                        eprintln!("{} Unable to parse file path: {:?}", "Err:".red(), source_path);
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
