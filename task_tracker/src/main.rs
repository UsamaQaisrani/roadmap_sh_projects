use crate::cli::Cli;

mod cli;
mod tasks;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = Cli::new();
    match cli.start() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
