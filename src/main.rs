mod commands;
mod models;
mod utils;
mod cli;

fn main() -> anyhow::Result<()> {
    let app = cli::build_cli();
    let matches = app.get_matches();

    if matches.is_present("help") {
        cli::print_help();
        return Ok(());
    }

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let component_name = add_matches.value_of("COMPONENT_NAME").unwrap();
        }
    }
}
