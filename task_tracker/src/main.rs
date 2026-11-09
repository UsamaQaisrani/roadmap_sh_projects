use crate::cli::Cli;

mod cli;
mod tasks;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = Cli::new();
    cli.start();
    Ok(())
}
