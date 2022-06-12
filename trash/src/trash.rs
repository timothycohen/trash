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

pub fn info(user_path: String) {
    let trash_names = TrashNames::from_trash_file_name(PathBuf::from(user_path));
    let trash_paths = AbsoluteTrashPaths::new(TrashDirPaths::new(), trash_names);
    trash_paths.guard_exists();
    TrashInfo::read_to_std(&trash_paths.trash_info_path);
}

pub fn info_all() {
    let trash_dir_paths = TrashDirPaths::new();
    for trash_info_path in trash_dir_paths.get_all_info_paths() {
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
    overwrite_guard(source_path);

    let trash_paths = match AbsoluteTrashPaths::find_by_source_path(source_path, &TrashDirPaths::new()) {
        Some(p) => p,
        None => {
            eprintln!("{} File not found in trash.", Colour::Blue.paint("Info:"));
            std::process::exit(1);
        }
    };

    trash_paths.restore_from_trash(source_path);
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
