use anyhow::Result;
use colored::*;

use crate::utils::project::get_cli_root;
use crate::utils::component::{get_available_components, load_component_config};
use crate::utils::ui::print_info;

pub fn list_components() -> Result<()> {
    let cli_root = get_cli_root();
    let template_dir = cli_root.join("templates");

    if !template_dir.exists() {
        println!("{} Templates directory not found!", "ERROR".red().bold());
        return Ok(());
    }

    // get all availble components
    let components = get_available_components(&template_dir)?;

    if components.is_empty() {
        print_info("No components found in this template.");
        return Ok(());
    }

    println!("{}", "Available components".bright_cyan().bold());
    println!("{}", "--------------------".bright_cyan());

    for component in components {
        //check for config files
        let config_path = template_dir.join(format!("{}.json", component));
        if let Some(config) = load_component_config(&config_path)? {
            let deps = config.dependencies.unwrap_or_default();
            let utils = config.utils.unwrap_or_default();

            if !deps.is_empty() || !utils.is_empty() {
                println!("{} {}", "•".yellow(), component.bright_white().bold());

                if !deps.is_empty() {
                    println!("  {} {}", "Dependencies".cyan(), deps.join(", "));
                }

                if !utils.is_empty() {
                    println!("  {} {}", "Utils".magenta(), utils.join(", "));
                }

                println!();
                continue;
            }
        }


        println!("{} {}", "•".yellow(), component);
    }

    println!("\nTo add a component, run: {} {}", "vin-ui add".bright_white(), "<component-name>".bright_green());
    Ok(())
}
