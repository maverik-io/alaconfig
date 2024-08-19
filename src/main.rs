use clap::Parser;

mod args;
mod handlers;
use args::*;
use handlers::*;

fn main() {
    let args = AlaconfigArgs::parse();
    match args.config {
        Config::Set(setter) => handle_set(setter),
        Config::Get(getter) => handle_get(getter),
    }
}
