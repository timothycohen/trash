use crate::GLOBAL;
use ansi_term::Colour;
use std::{
    ffi::OsString,
    io,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct TrashPath {
    pub trash_files_dir: PathBuf,
    pub trash_info_dir: PathBuf,
}

impl TrashPath {
    pub fn new() -> TrashPath {
        let home = match home::home_dir() {
            None => {
                eprintln!("{} Unable to get home directory.", Colour::Red.paint("Err:"));
                std::process::exit(1);
            }
            Some(h) => h,
        };

        let trash_path: PathBuf = [&home, &PathBuf::from(".local/share/Trash")].iter().collect();
        let trash_info_dir: PathBuf = [&home, &PathBuf::from(".local/share/Trash/info")].iter().collect();
        let trash_files_dir: PathBuf = [&home, &PathBuf::from(".local/share/Trash/files")].iter().collect();

        if !trash_info_dir.exists() | !trash_files_dir.exists() {
            if GLOBAL.verbose() {
                println!(
                    "{} Couldn't find Trash. Creating at {:?}",
                    Colour::Blue.paint("Info:"),
                    trash_path
                );
            }
            if let Err(e) = Self::create_default_trash_dir(&trash_info_dir, &trash_files_dir) {
                eprintln!("{} Unable to create Trash. {}", Colour::Yellow.paint("Hint:"), e);
                std::process::exit(1);
            };
        }

        TrashPath {
            trash_files_dir,
            trash_info_dir,
        }
    }

    fn create_default_trash_dir(trash_info_path: &Path, trash_files_path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(trash_info_path)?;
        std::fs::create_dir_all(trash_files_path)?;
        Ok(())
    }

    /// Returns (trash_info_path, trash_file_path)
    pub fn create_trash_file_names(&self, file_basename: OsString, id: Uuid) -> (PathBuf, PathBuf) {
        let file_basename = file_basename.to_string_lossy().to_owned();

        let info_file_name = format!("{}.{:?}.trashinfo", file_basename, id);
        let mut trash_info_path = self.trash_info_dir.clone();
        trash_info_path.push(info_file_name);

        let files_file_name = format!("{}.{:?}", file_basename, id);
        let mut trash_file_path = self.trash_files_dir.clone();
        trash_file_path.push(files_file_name);

        (trash_info_path, trash_file_path)
    }
}

impl Default for TrashPath {
    fn default() -> Self {
        Self::new()
    }
}
