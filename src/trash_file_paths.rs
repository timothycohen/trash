use crate::{
    global::GLOBAL, trash::overwrite_guard, trash_dir_paths::TrashDirPaths, trash_info::TrashInfo,
    trash_names::TrashNames,
};
use ansi_term::Colour;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct AbsoluteTrashPaths {
    pub trash_info_path: PathBuf,
    pub trash_file_path: PathBuf,
}

pub enum AbsoluteTrashPathsError {
    WriteInfo(std::io::Error),
    TrashFile(std::io::Error, AbsoluteTrashPaths),
    RestoreFile(std::io::Error),
    DeleteInfo(std::io::Error),
}

pub type AbsoluteTrashPathsResult = Result<(), AbsoluteTrashPathsError>;

impl AbsoluteTrashPaths {
    pub fn new(trash_paths: TrashDirPaths, trash_names: TrashNames) -> Self {
        let mut trash_info_path = trash_paths.trash_info_dir;
        trash_info_path.push(trash_names.trash_info_name);

        let mut trash_file_path = trash_paths.trash_files_dir;
        trash_file_path.push(trash_names.trash_file_name);

        AbsoluteTrashPaths {
            trash_info_path,
            trash_file_path,
        }
    }

    pub fn guard_exists(&self) {
        if !self.trash_file_path.exists() {
            eprintln!(
                "{} Trash file path does not exist: {:?}",
                Colour::Red.paint("Error:"),
                self.trash_file_path
            );
            std::process::exit(1);
        }
        if !self.trash_info_path.exists() {
            eprintln!(
                "{} Trash info path does not exist: {:?}",
                Colour::Red.paint("Error:"),
                self.trash_info_path
            );
            std::process::exit(1);
        }
    }

    // todo handle multiple files with the same source path
    fn find_file_info(source_path: &Path, trash_info_dir: &Path) -> Option<PathBuf> {
        let path_string = TrashInfo::format_source_path(source_path);

        if GLOBAL.verbose() {
            println!(
                "{} Checking trash info files for the path_string {}",
                Colour::Blue.paint("Info:"),
                path_string
            );
        }

        for entry in fs::read_dir(trash_info_dir).expect("Path checked in TrashDirPaths.") {
            let entry = entry.ok()?;
            let path = entry.path();
            let extension = Path::new("e.trashinfo").extension();

            if path.is_file() && path.extension() == extension {
                let file = fs::read_to_string(&path).ok()?;
                let mut lines = file.lines();
                // move down to Path
                lines.next();
                lines.next();
                if let Some(path_str) = lines.next() {
                    if path_str == path_string {
                        return Some(path);
                    }
                }
            }
        }
        None
    }

    pub fn find_by_source_path(source_path: &Path, trash_path: &TrashDirPaths) -> Option<AbsoluteTrashPaths> {
        let trash_info_path = AbsoluteTrashPaths::find_file_info(source_path, &trash_path.trash_info_dir)?;
        let mut trash_file_path = trash_path.trash_files_dir.to_owned();
        trash_file_path.push(trash_info_path.file_stem().expect("file has .trashinfo ext"));

        Some(AbsoluteTrashPaths {
            trash_info_path,
            trash_file_path,
        })
    }

    pub fn try_delete_info_file(&self) -> AbsoluteTrashPathsResult {
        if GLOBAL.verbose() {
            println!(
                "{} Removing info file at {}",
                Colour::Blue.paint("Info:"),
                self.trash_info_path.display()
            );
        }

        std::fs::remove_file(&self.trash_info_path).map_err(AbsoluteTrashPathsError::DeleteInfo)
    }

    pub fn delete_info_file(&self) {
        Self::clean_and_bail_on_error(self.try_delete_info_file());
    }

    pub fn try_move_to_trash(self, source_path: &Path) -> AbsoluteTrashPathsResult {
        if GLOBAL.verbose() {
            println!(
                "{} Moving trashed file to {}",
                Colour::Blue.paint("Info:"),
                self.trash_file_path.display()
            );
        }

        std::fs::rename(source_path, &self.trash_file_path).map_err(|e| AbsoluteTrashPathsError::TrashFile(e, self))
    }

    /// Consumes self to clean up if necessary
    pub fn move_to_trash(self, source_path: &Path) {
        Self::clean_and_bail_on_error(self.try_move_to_trash(source_path));
    }

    pub fn try_write_info_file(&self, content: String) -> AbsoluteTrashPathsResult {
        overwrite_guard(&self.trash_info_path);

        if GLOBAL.verbose() {
            println!(
                "{} Writing info file to {}",
                Colour::Blue.paint("Info:"),
                self.trash_info_path.display()
            );
        }

        std::fs::write(&self.trash_info_path, content).map_err(AbsoluteTrashPathsError::WriteInfo)
    }

    pub fn write_info_file(&self, content: String) {
        Self::clean_and_bail_on_error(self.try_write_info_file(content));
    }

    pub fn clean_and_bail_on_error(maybe_error: AbsoluteTrashPathsResult) {
        if let Err(trash_paths_error) = maybe_error {
            match trash_paths_error {
                AbsoluteTrashPathsError::DeleteInfo(e) => {
                    eprintln!("{} Could not remove info file. {:?}", Colour::Red.paint("Err:"), e);
                    std::process::exit(1)
                }
                AbsoluteTrashPathsError::WriteInfo(e) => {
                    eprintln!("{} Could not write info file. {:?}", Colour::Red.paint("Err:"), e);
                    std::process::exit(1)
                }
                AbsoluteTrashPathsError::TrashFile(e, trash_paths) => {
                    eprintln!("{} Could not move trashed file. {:?}", Colour::Red.paint("Err:"), e);
                    trash_paths.delete_info_file();
                    std::process::exit(1)
                }
                AbsoluteTrashPathsError::RestoreFile(e) => {
                    eprintln!("{} Could not restore trashed file. {:?}", Colour::Red.paint("Err:"), e);
                    std::process::exit(1)
                }
            }
        }
    }

    pub fn try_restore_from_trash(&self, source_path: &Path) -> AbsoluteTrashPathsResult {
        if GLOBAL.verbose() {
            println!(
                "{} Restoring trashed file to {}",
                Colour::Blue.paint("Info:"),
                source_path.display()
            );
        }

        std::fs::rename(&self.trash_file_path, source_path).map_err(AbsoluteTrashPathsError::RestoreFile)
    }

    pub fn restore_from_trash(&self, source_path: &Path) {
        Self::clean_and_bail_on_error(self.try_restore_from_trash(source_path));
        self.delete_info_file();
    }
}
