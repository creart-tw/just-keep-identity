use clap::Parser;
use jkim::{preprocess_args, Cli};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = preprocess_args(args);

    let cli = Cli::parse_from(args);
    if let Err(e) = jkim::run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
