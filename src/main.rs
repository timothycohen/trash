use args::{Args, Method::*};
use clap::Parser;
use colored::Colorize;
use config::Config;
use global::GLOBAL;
use trash::{empty, info, info_all, info_wild_card, put, restore};

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
        Put => {
            path_arg_guard(&args.files);
            for user_path in args.files {
                put(Config::new(&args.method, user_path))
            }
        }
        Restore => {
            // todo. if no path, do an interactive restoration by showing the trash contents
            path_arg_guard(&args.files);
            for user_path in args.files {
                restore(&Config::new(&args.method, user_path).source_path)
            }
        }
        Empty => empty(),
        Info => match args.files.len() {
            0 => info_all(),
            _ => {
                for user_path in args.files {
                    if args.all {
                        info_wild_card(&user_path);
                    } else {
                        info(&user_path)
                    }
                }
            }
        },
    };
}

pub fn path_arg_guard(user_path: &[String]) {
    if user_path.is_empty() {
        eprintln!("{} The following arguments are required:", "error:".red());
        eprintln!("    {}\n", "[FILE]".green());
        eprintln!("For more information try {}", "--help".green());
        std::process::exit(1)
    }
}
