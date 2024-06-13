use crate::GLOBAL;
use colored::Colorize;
use std::{
    fs::ReadDir,
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
/// The absolute paths to the trash directory's file and info directories
pub struct TrashDirPaths {
    pub trash_files_dir: PathBuf,
    pub trash_info_dir: PathBuf,
}

impl TrashDirPaths {
    pub fn new() -> TrashDirPaths {
        let home = match home::home_dir() {
            None => {
                eprintln!("{} Unable to get home directory.", "Err:".red());
                std::process::exit(1);
            }
            Some(h) => h,
        };

        let trash_path: PathBuf = [&home, &PathBuf::from(".local/share/Trash")].iter().collect();
        let trash_info_dir: PathBuf = [&home, &PathBuf::from(".local/share/Trash/info")].iter().collect();
        let trash_files_dir: PathBuf = [&home, &PathBuf::from(".local/share/Trash/files")].iter().collect();

        if !trash_info_dir.exists() | !trash_files_dir.exists() {
            if GLOBAL.verbose() {
                println!("{} Couldn't find Trash. Creating at {:?}", "Info:".blue(), trash_path);
            }
            if let Err(e) = Self::create_default_dirs(&trash_info_dir, &trash_files_dir) {
                eprintln!("{} Unable to create Trash. {}", "Hint:".yellow(), e);
                std::process::exit(1);
            };
        }

        TrashDirPaths {
            trash_files_dir,
            trash_info_dir,
        }
    }

    fn create_default_dirs(trash_info_path: &Path, trash_files_path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(trash_info_path)?;
        std::fs::create_dir_all(trash_files_path)?;
        Ok(())
    }

    fn remove_default_dirs(trash_info_path: &Path, trash_files_path: &Path) -> io::Result<()> {
        std::fs::remove_dir_all(trash_info_path)?;
        std::fs::remove_dir_all(trash_files_path)?;
        Ok(())
    }

    /// Writes to stderr and bails on failure
    pub fn empty() {
        let trash_path = TrashDirPaths::default();

        let mut number_of_files = std::fs::read_dir(&trash_path.trash_files_dir)
            .expect("Path created in default.")
            .count();
        if number_of_files == 0 {
            number_of_files = std::fs::read_dir(&trash_path.trash_info_dir)
                .expect("Path created in default.")
                .count();
        };

        if number_of_files == 0 && !GLOBAL.force() {
            println!("{} {} is empty", "Info:".blue(), &trash_path.trash_files_dir.display());
            std::process::exit(0);
        }

        if !GLOBAL.force() {
            let answer = dialoguer::Confirm::new()
                .with_prompt(format!(
                    "{} Permanently delete all {} files at {}?",
                    "Warn:".red(),
                    number_of_files,
                    &trash_path.trash_files_dir.display()
                ))
                .interact_opt()
                .expect("User answer succeeds");

            match answer {
                Some(true) => (),
                _ => std::process::exit(0),
            }
        }

        if GLOBAL.verbose() {
            println!(
                "{} Deleting {} files in {:?}",
                "Info:".blue(),
                number_of_files,
                &trash_path.trash_files_dir
            );
        }

        if let Err(e) = Self::remove_default_dirs(&trash_path.trash_info_dir, &trash_path.trash_files_dir) {
            eprintln!("{} Unable to remove Trash. {}", "Err:".red(), e);
        } else if let Err(e) = Self::create_default_dirs(&trash_path.trash_info_dir, &trash_path.trash_files_dir) {
            eprintln!("{} Unable to recreate default Trash. {}", "Err:".red(), e);
        }
    }

    pub fn get_all_info_paths(&self) -> ReadDir {
        std::fs::read_dir(&self.trash_info_dir).expect("Permissions and existence checked in new()")
    }
}

impl Default for TrashDirPaths {
    fn default() -> Self {
        Self::new()
    }
}
