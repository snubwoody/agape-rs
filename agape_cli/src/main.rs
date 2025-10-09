use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    #[command()]
    /// Run your application.
    Run,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run => run_app(),
    }
}

fn run_app() {
    let cmd = std::process::Command::new("cargo")
        .args(&["run"])
        .status()
        .expect("Failed to run app");
}
