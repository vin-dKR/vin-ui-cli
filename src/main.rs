mod commands;
mod models;
mod utils;
mod cli;

fn main() -> anyhow::Result<()> {
    let app = cli::build_cli();
    let matches = app.get_matches();
}
