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

fn main() {
    let args = Args::parse();
    GLOBAL.set_verbose(args.verbose);

    match args.method {
        Put => put(Config::from(args), TrashPath::new()),
        Restore => restore(Config::from(args), TrashPath::new()),
        Empty => empty(TrashPath::new()),
    };
}
