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

    pub fn delete_info_file(&self) {
        if GLOBAL.verbose() {
            println!(
                "{} Removing info file at {}",
                Colour::Blue.paint("Info:"),
                self.trash_info_path.display()
            );
        }

        if let Err(e) = std::fs::remove_file(&self.trash_info_path) {
            eprintln!("{} Could not remove info file. {:?}", Colour::Red.paint("Err:"), e);
            std::process::exit(1)
        }
    }

    pub fn move_to_trash(&self, source_path: &Path) {
        if GLOBAL.verbose() {
            println!(
                "{} Moving trashed file to {}",
                Colour::Blue.paint("Info:"),
                self.trash_file_path.display()
            );
        }

        if let Err(e) = std::fs::rename(source_path, &self.trash_file_path) {
            eprintln!("{} Could not move trashed file. {:?}", Colour::Red.paint("Err:"), e);

            self.delete_info_file();
            std::process::exit(1)
        }
    }

    pub fn write_info_file(&self, content: String) {
        overwrite_guard(&self.trash_info_path);

        if GLOBAL.verbose() {
            println!(
                "{} Writing info file to {}",
                Colour::Blue.paint("Info:"),
                self.trash_info_path.display()
            );
        }

        if let Err(e) = std::fs::write(&self.trash_info_path, content) {
            eprintln!("{} Could not write info file. {:?}", Colour::Red.paint("Err:"), e);
            std::process::exit(1)
        }
    }
}
