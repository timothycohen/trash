use crate::{config::Config, trash_path::TrashPath, GLOBAL};
use ansi_term::Colour;
use chrono::{DateTime, Utc};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn put(config: Config, trash_path: TrashPath) {
    let id = config.uuid.expect("UUID was set in config");
    let (trash_info_path, trash_file_path) = trash_path.create_trash_file_names(config.file_basename, id);

    overwrite_guard(&trash_file_path);
    write_info_file(&trash_info_path, &config.source_path);
    move_to_trash(&trash_info_path, &trash_file_path, &config.source_path);
}

pub fn restore(config: Config, trash_path: TrashPath) {
    overwrite_guard(&config.source_path);
    let (trash_info_path, trash_file_path) = find_file_paths_in_trash(&config, &trash_path);
    restore_from_trash(&trash_info_path, &trash_file_path, &config.source_path);
}

pub fn empty(trash_path: TrashPath) {
    let number_of_files = std::fs::read_dir(&trash_path.trash_files_dir)
        .expect("Path checked in config.")
        .count();

    if number_of_files == 0 && !GLOBAL.force() {
        println!(
            "{} {} is empty",
            Colour::Blue.paint("Info:"),
            &trash_path.trash_files_dir.display()
        );
        std::process::exit(0);
    }

    if !GLOBAL.force() {
        let answer = dialoguer::Confirm::new()
            .with_prompt(format!(
                "{} Permanently delete all {} files at {}?",
                Colour::Red.paint("Warn:"),
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
            Colour::Blue.paint("Info:"),
            number_of_files,
            &trash_path.trash_files_dir
        );
    }

    if let Err(e) = std::fs::remove_dir_all(&trash_path.trash_files_dir) {
        eprintln!("{}", e);
    };
    if let Err(e) = std::fs::remove_dir_all(&trash_path.trash_info_dir) {
        eprintln!("{}", e);
    };
    std::fs::create_dir(&trash_path.trash_files_dir).expect("restoring after deleting has permission");
    std::fs::create_dir(&trash_path.trash_info_dir).expect("restoring after deleting has permission");
}

fn overwrite_guard(path: &Path) {
    if path.exists() && !GLOBAL.force() {
        eprintln!("{} Will not overwrite file: {:?}", Colour::Red.paint("Err:"), path);
        std::process::exit(1)
    }
}

fn delete_info_file(trash_info_path: &Path) {
    if GLOBAL.verbose() {
        println!(
            "{} Removing info file at {}",
            Colour::Blue.paint("Info:"),
            trash_info_path.display()
        );
    }

    if let Err(e) = std::fs::remove_file(trash_info_path) {
        eprintln!("{} Could not remove info file. {:?}", Colour::Red.paint("Err:"), e);
        std::process::exit(1)
    }
}

fn write_info_file(trash_info_path: &Path, source_path: &Path) {
    let content = format!(
        "{}\n{}\n{}",
        "[Trash Info]",
        create_trashinfo_path(source_path),
        create_trashinfo_timestamp()
    );

    if GLOBAL.verbose() {
        println!(
            "{} Writing info file to {}",
            Colour::Blue.paint("Info:"),
            trash_info_path.display()
        );
    }

    if let Err(e) = std::fs::write(trash_info_path, content) {
        eprintln!("{} Could not write info file. {:?}", Colour::Red.paint("Err:"), e);
        std::process::exit(1)
    }
}

fn move_to_trash(trash_info_path: &Path, trash_file_path: &Path, source_path: &Path) {
    if GLOBAL.verbose() {
        println!(
            "{} Moving trashed file to {}",
            Colour::Blue.paint("Info:"),
            trash_file_path.display()
        );
    }

    if let Err(e) = std::fs::rename(source_path, trash_file_path) {
        eprintln!("{} Could not move trashed file. {:?}", Colour::Red.paint("Err:"), e);

        delete_info_file(trash_info_path);
        std::process::exit(1)
    }
}

fn restore_from_trash(trash_info_path: &Path, trash_file_path: &Path, source_path: &Path) {
    if GLOBAL.verbose() {
        println!(
            "{} Restored trashed file to {}",
            Colour::Blue.paint("Info:"),
            source_path.display()
        );
    }

    if let Err(e) = std::fs::rename(trash_file_path, source_path) {
        eprintln!("{} Could not restore trashed file. {:?}", Colour::Red.paint("Err:"), e);
        std::process::exit(1)
    }

    delete_info_file(trash_info_path);
}

// todo handle multiple files with the same source path
fn find_file_info_in_trash(source_path: &Path, trash_info_dir: &Path) -> Option<PathBuf> {
    let path_string = create_trashinfo_path(source_path);
    if GLOBAL.verbose() {
        println!(
            "{} Checking trash info files for the path_string {}",
            Colour::Blue.paint("Info:"),
            path_string
        );
    }

    for entry in fs::read_dir(trash_info_dir).expect("Path checked in TrashPath.") {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.is_file() && path.extension() == Path::new("e.trashinfo").extension() {
            let file = fs::read_to_string(&path).ok()?;
            let mut lines = file.lines();
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

fn create_trashinfo_path(source_path: &Path) -> String {
    format!("Path={}", source_path.display())
}

fn create_trashinfo_timestamp() -> String {
    let dt: DateTime<Utc> = std::time::SystemTime::now().into();
    format!("DeletionDate={:?}", dt.to_rfc3339())
}

fn find_file_paths_in_trash(config: &Config, trash_path: &TrashPath) -> (PathBuf, PathBuf) {
    let trash_info_path = match find_file_info_in_trash(&config.source_path, &trash_path.trash_info_dir) {
        Some(f) => f,
        None => {
            eprintln!("{} File not found in trash.", Colour::Blue.paint("Info:"));
            std::process::exit(1);
        }
    };
    let mut trash_file_path = trash_path.trash_files_dir.to_owned();
    trash_file_path.push(trash_info_path.file_stem().expect("file has .trashinfo ext"));

    (trash_info_path, trash_file_path)
}
