use clap::Parser;
use jkim::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = jkim::run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
