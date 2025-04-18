mod commands;
mod models;
mod utils;
mod cli;

use colored::*;
use utils::template_finder;

fn main() -> anyhow::Result<()> {
    // Check if templates exist, install if needed
    let templates_dir = template_finder::get_templates_dir();
    if !templates_dir.exists() {
        println!("{} Templates directory not found.", "INFO:".blue().bold());
        println!("Installing templates to configuration directory...");
        template_finder::install_templates()?;
    }


    let app = cli::build_cli();
    let matches = app.get_matches();

    // Handle top-level --help or -h flag
    if matches.get_flag("help") {
        cli::print_help();
        return Ok(());
    }

    // Handle subcommands
    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let component_name = add_matches.get_one::<String>("COMPONENT_NAME").unwrap();
            commands::add::add_component(component_name).expect("Skill issues on adding compo");
        }
        Some(("list", _)) => commands::list::list_components()?,
        Some(("init", _)) => commands::init::init_components_directory()?,
        Some(("help", _)) => cli::print_help(), // Explicitly handle help subcommand
        _ => cli::print_help(), // Default case for unrecognized subcommands
    }

    Ok(())
}
