use crate::{config::Config, trash_path::TrashPath, GLOBAL};
use ansi_term::Colour;
use chrono::{DateTime, Utc};
use std::path::Path;

fn delete_info_file(trash_info_path: &Path) {
    if GLOBAL.verbose() {
        println!(
            "{} Removing info file. {}",
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
    let header = "[Trash Info]";
    let path = format!("Path={}", source_path.display());
    let dt: DateTime<Utc> = std::time::SystemTime::now().into();
    let dd = format!("DeletionDate={:?}", dt.to_rfc3339());
    let content = format!("{}\n{}\n{}", header, path, dd);

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

fn move_trashed_file(trash_info_path: &Path, trash_file_path: &Path, source_path: &Path) {
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

pub fn put(config: Config, trash_path: TrashPath) {
    let id = config.uuid.expect("UUID was set in config");
    let (trash_info_path, trash_file_path) = trash_path.create_trash_file_names(config.file_basename, id);

    if trash_file_path.exists() {
        eprintln!(
            "{} Will not overwrite file: {:?}",
            Colour::Red.paint("Err:"),
            trash_file_path
        );
        std::process::exit(1)
    };

    write_info_file(&trash_info_path, &config.path);
    move_trashed_file(&trash_info_path, &trash_file_path, &config.path);
}

pub fn restore(config: Config, trash_path: TrashPath) {
    println!("{:?}", config);
    println!("{:?}", trash_path);
    todo!()
}

pub fn empty(trash_path: TrashPath) {
    let number_of_files = std::fs::read_dir(&trash_path.trash_files_dir)
        .expect("Path checked in config.")
        .count();

    if number_of_files == 0 {
        println!(
            "{} {} is empty",
            Colour::Blue.paint("Info:"),
            &trash_path.trash_files_dir.display()
        );
        std::process::exit(0);
    }

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
