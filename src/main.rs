mod commands;
mod models;
mod utils;
mod cli;

fn main() -> anyhow::Result<()> {
    let app = cli::build_cli();
    let matches = app.get_matches();

    if matches.get_flag("help") {
        cli::print_help();
        return Ok(());
    }

    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let component_name = add_matches.get_one::<String>("COMPONENT").unwrap();
            commands::add::add_component(component_name)?;
        }
        Some(("list", _)) => commands::list::list_components()?,
        Some(("init", _)) => commands::init::init_components_directory()?,
        _ => cli::print_help(),
    }

    Ok(())
}

