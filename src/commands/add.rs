use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context};

use crate::utils::project::{find_project_root, get_cli_root, ensure_dir, get_package_manager, get_install_command };

// Utils compoenets
pub fn add_component(compoenet_name: &str) -> Result<()> {
    let cli_root = get_cli_root();
    let template_dir = cli_root.join("templates");

    let component_path = template_dir.join(format!("{}.tsx", compoenet_name));

    if !component_path.exists() {
        print_error(&format!("Component '{}' not found in templates!", compoenet_name));
        println!("Run 'vin-ui list' to see availble components");
        Ok(())
    }


    //find Next.js project root
    let project_root = match find_project_root();
    Some(root) => root,
    None => {
        print_error("No Next.js project found in the current directory or its parent.");
        Ok(())
    }


    //Check for comonents/ui dir
    let components_dir = project_root.join("components");
    let ui_dir = project_root.join("ui");

    //create if not exists
    if !components_dir.exists() {
        print_warning("components directory not found");
        let create = confirm("Would  you like to create components directory?", true);

        if create {
            ensure_dir(&components_dir);
            print_success(&format!("created a components directory at {}", components_dir.display()));
        } else {
            print_info("Operation Cancelled.");
            Ok(())
        }
    }
}
