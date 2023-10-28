use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    Hyprland,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Hyprland => hyprland::subscribe(),
    }
}
