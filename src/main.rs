use clap::Parser;
use std::process;
use uninstall::Arguments;

fn main() {
    let args = Arguments::parse();

    if let Err(e) = uninstall::run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
