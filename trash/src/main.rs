use args::{Args, Method::*};
use clap::Parser;
use config::Config;
use global::GLOBAL;
use trash::{empty, info, info_all, put, restore};

mod args;
mod config;
mod global;
mod trash;
mod trash_dir_paths;
mod trash_file_paths;
mod trash_info;
mod trash_names;

/// ## Examples
///
/// `trash empty` Empties the trash directory.
///
/// `trash info file` Reads the info of a file in ~/.local/share/Trash/files
///
/// `trash put file -v` Trashes a file and prints verbose logs.
///
/// `trash restore file -f` Restores a file, potentially forcing an overwrite.

fn main() {
    let args = Args::parse();
    GLOBAL.set_verbose(args.verbose);
    GLOBAL.set_force(args.force);

    match args.method {
        Put => put(Config::from(args)),
        Restore => restore(&Config::from(args).source_path),
        Empty => empty(),
        Info => match args.file {
            Some(f) => info(f),
            None => info_all(),
        },
    };
}
