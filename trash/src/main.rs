use args::{Args, Method::*};
use clap::Parser;
use config::Config;
use global::GLOBAL;
use trash::{empty, put, restore};
use trash_path::TrashPath;

mod args;
mod config;
mod global;
mod trash;
mod trash_path;

/// ## Examples
///
/// `trash empty` Empties the trash directory.
///
/// `trash put file -v` Trashes a file and prints verbose logs.
///
/// `trash restore file -f` Restores a file, potentially forcing an overwrite.

fn main() {
    let args = Args::parse();
    GLOBAL.set_verbose(args.verbose);
    GLOBAL.set_force(args.force);

    match args.method {
        Put => put(Config::from(args), TrashPath::new()),
        Restore => restore(Config::from(args), TrashPath::new()),
        Empty => empty(TrashPath::new()),
    };
}
