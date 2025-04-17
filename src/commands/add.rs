use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context};

use crate::utils::project::{find_project_root, get_cli_root, ensure_dir, get_package_manager, get_install_command };
use crate::utils::ui::{print_success, print_error, print_warning, print_info, confirm};

// Utils compoenets
pub fn add_component(component_name: &str) -> Result<()> {
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
            ensure_dir(&components_dir)?;
            print_success(&format!("created a components directory at {}", components_dir.display()));
        } else {
            print_info("Operation Cancelled.");
            Ok(())
        }
    }


    //for ui directory
    if !ui_dir.exists() {
        print_warning("componens/ui directory not found");
        let create = confirm("Would  you like to create ui directory?", true);

        if create {
            ensure_dir(&ui_dir)?;
            print_success(&format!("created a components directory at {}", ui_dir.display()));
        } else {
            print_info("Operation Cancelled.");
            Ok(())
        }
    }


    let dest_file = ui_dir.join(format!("{}.tsx", component_name));

    if dest_file.exists() {
        let overwrite = confirm(
            &format!("component {} already exists, Overwrite?", component_name),
            false
        )

        if !overwrite {
            print_info("Operation Cancelled.");
            Ok(())
        }
    }

    //copy the component file
    fs::copy(&component_path, &dest_file)
        .context(format!("Failed to copy component file to {}", dest_file.display()))?;

    print_success(&format!(
            "Component '{}' successfully installed to {}",
            component_name,
            dest_file.display()
    ));

    //Check for component config file that specifies dependencies
    let config_path = template_dir.join(format!("{}.json", component_name));
}
