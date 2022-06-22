use crate::{
    config::Config, trash_dir_paths::TrashDirPaths, trash_file_paths::AbsoluteTrashPaths, trash_info::TrashInfo,
    trash_names::TrashNames, GLOBAL,
};
use ansi_term::Colour;
use std::path::{Path, PathBuf};

pub fn put(config: Config) {
    let trash_names = TrashNames::from_base_name(config.file_basename);
    let trash_file_name = trash_names.trash_file_name.clone();
    let trash_paths = AbsoluteTrashPaths::new(TrashDirPaths::new(), trash_names);
    let content = TrashInfo::new(config.source_path.clone(), trash_file_name).content();
    trash_paths.write_info_file(content);
    trash_paths.move_to_trash(&config.source_path);
}

/// Print .trashinfo data if it exists
pub fn info(user_path: &str) {
    let trash_names = TrashNames::from_trash_file_name(PathBuf::from(user_path));
    let trash_paths = AbsoluteTrashPaths::new(TrashDirPaths::new(), trash_names);
    trash_paths.guard_exists();
    TrashInfo::read_to_std(&trash_paths.trash_info_path);
}

/// Print all .trashinfo data with a matching file name
pub fn info_wild_card(user_path: &str) {
    let trash_names = TrashNames::from_trash_file_name(PathBuf::from(user_path));
    let mut partial_name = AbsoluteTrashPaths::new(TrashDirPaths::new(), trash_names).trash_info_path;
    partial_name.set_extension("");
    let user_path = partial_name.to_string_lossy().into_owned();

    for trash_info_path in TrashDirPaths::new().get_all_info_paths() {
        match trash_info_path {
            Ok(p) => {
                let p = p.path();
                let info_path = p.to_string_lossy();
                if info_path.contains(&user_path) {
                    TrashInfo::read_to_std(&p)
                }
            }
            Err(e) => {
                eprintln!("{} Unable to read file: {}", Colour::Red.paint("Error:"), e);
            }
        };
    }
}

/// Print all .trashinfo data
pub fn info_all() {
    for trash_info_path in TrashDirPaths::new().get_all_info_paths() {
        match trash_info_path {
            Ok(p) => {
                println!();
                TrashInfo::read_to_std(&p.path())
            }
            Err(e) => {
                eprintln!("{} Unable to read file: {}", Colour::Red.paint("Error:"), e);
            }
        };
    }
}

pub fn restore(source_path: &Path) {
    let trash_paths = match AbsoluteTrashPaths::find_by_source_path(source_path, &TrashDirPaths::new()) {
        Some(p) => p,
        None => {
            eprintln!(
                "{} File not found in trash. {:?}",
                Colour::Blue.paint("Info:"),
                source_path
            );
            std::process::exit(1);
        }
    };

    // Use the .trashinfo source_path instead of the user's to allow for partial matches
    let trash_info = match TrashInfo::from_file(&trash_paths.trash_info_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!(
                "{} .trashinfo file has been corrupted. {:?}",
                Colour::Yellow.paint("Warning:"),
                e
            );
            std::process::exit(1);
        }
    };

    overwrite_guard(&trash_info.source_path);
    trash_paths.restore_from_trash(&trash_info.source_path);
}

pub fn empty() {
    TrashDirPaths::empty();
}

pub fn overwrite_guard(path: &Path) {
    if path.exists() && !GLOBAL.force() {
        eprintln!("{} Will not overwrite file: {:?}", Colour::Red.paint("Err:"), path);
        std::process::exit(1)
    }
}
