use std::fs;
use colored::*;
use std::process::Command;
use anyhow::{Result, Context};

use crate::utils::component::{load_component_config, add_utility};
use crate::utils::project::{find_project_root, get_cli_root, ensure_dir, get_package_manager, get_install_command };
use crate::utils::ui::{print_success, print_error, print_warning, print_info, confirm};

// Utils compoenets
pub fn add_component(component_name: &str) -> Result<()> {
    let cli_root = get_cli_root();
    let template_dir = cli_root.join("templates");

    let component_path = template_dir.join(format!("{}.tsx", component_name));
    let config_path = template_dir.join(format!("{}.json", component_name));

    if !component_path.exists() && !config_path.exists() {
        print_error(&format!("Neither component '{}' nor its config found in templates!", component_name));
        println!("Run 'vin-ui list' to see available components");
        return Ok(());
    }

    if !component_path.exists() {
        print_error(&format!("Component '{}' not found in templates!", component_name));
        println!("Run 'vin-ui list' to see availble components");
        return Ok(());
    }


    //find Next.js project root
    let project_root = match find_project_root() {
        Some(root) => root,
        None => {
            print_error("No Next.js project found in the current directory or its parent.");
            return Ok(());
        }
    };

    //Check for comonents/ui dir
    let components_dir = project_root.join("components");
    let ui_dir = components_dir.join("ui");
    let lib_dir = project_root.join("lib");

    //create if not exists
    if !components_dir.exists() {
        print_warning("components directory not found");
        let create = confirm("Would  you like to create components directory?", true);

        if create {
            ensure_dir(&components_dir)?;
            print_success(&format!("created a components directory at {}", components_dir.display()));
        } else {
            print_info("Operation Cancelled.");
            return Ok(());
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
            return Ok(());
        }
    }


    let dest_file = ui_dir.join(format!("{}.tsx", component_name));

    if dest_file.exists() {
        let overwrite = confirm(
            &format!("component {} already exists, Overwrite?", component_name),
            false
        );

            if !overwrite {
                print_info("Operation Cancelled.");
                return Ok(());
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
    if let Ok(Some(config)) = load_component_config(&config_path) {
        // check if d comp. has dependencies
        if let Some(dependencies) = config.dependencies {
            if !dependencies.is_empty() {
                print_info(&format!(
                        "Component '{}' requires the following dependencies:",
                        component_name
                ));

                for dep in &dependencies {
                    println!("  - {}", dep);
                }

                let insall_deps = confirm("Would ya like to install these deps..?", true);

                if insall_deps {
                    let package_manager = get_package_manager(&project_root);
                    let install_arg = get_install_command(package_manager);

                    print_info("Installing deps..");

                    let status = Command::new(package_manager)
                        .current_dir(&project_root)
                        .arg(install_arg)
                        .args(&dependencies)
                        .status()
                        .context("Failed to run install comands")?;

                    if status.success() {
                        print_success("Yoo hoo, Deps Installed!");
                    } else {
                        print_error("Sorry :( Failed to install Deps..");
                    }
                }
            }
        }

        // Check if the component requires utility functions
        if let Some(utils) = config.utils {
            if !utils.is_empty() {
                print_info(&format!(
                        "Component {} requires the following utilities.",
                        component_name
                ));

                for util in &utils {
                    println!("-  {}", util);
                }

                let install_utils = confirm("Would ya like to install these following utilities?", true);

                if install_utils {
                    // Create lib/utils.ts if it doesn't exist
                    let lib_dir = project_root.join("lib");
                    ensure_dir(&lib_dir)?;

                    // Add each utility functions
                    for util in &utils {
                        add_utility(&lib_dir, &util, &template_dir)?;
                    }

                    print_success(&format!(
                            "utilities functions added to {}",
                            lib_dir.join("utils.ts").display()
                    ));
                }
            }
        } 


        if let Some(additional_files) = config.additional_files {
            print_info(&format!(
                "Component '{}' requires the following additional files:",
                component_name
            ));
            
            for file_info in &additional_files {
                println!("  - {}", file_info.source);
                
                // Determine destination based on file extension
                let source_path = template_dir.join("utils").join(&file_info.source);
                let dest_path = if file_info.source.ends_with(".tsx") || 
                                  file_info.source.ends_with(".css") ||
                                  file_info.source.ends_with(".scss") {
                    // Component files go to components/ui
                    ui_dir.join(&file_info.dest.clone().unwrap_or(file_info.source.clone()))
                } else if file_info.source.ends_with(".ts") {
                    // Utility files go to lib
                    ensure_dir(&lib_dir)?;
                    lib_dir.join(&file_info.dest.clone().unwrap_or(file_info.source.clone()))
                } else {
                    // Other files use specified destination or default to components/ui
                    ui_dir.join(&file_info.dest.clone().unwrap_or(file_info.source.clone()))
                };
                
                // Create parent directories if needed
                if let Some(parent) = dest_path.parent() {
                    ensure_dir(parent)?;
                }
                
                // Copy the file
                if source_path.exists() {
                    fs::copy(&source_path, &dest_path)
                        .context(format!("Failed to copy {} to {}", source_path.display(), dest_path.display()))?;
                    print_success(&format!(
                        "Additional file '{}' installed to {}",
                        file_info.source,
                        dest_path.display()
                    ));
                } else {
                    print_warning(&format!("Additional file '{}' not found in templates/utils", file_info.source));
                }
            }
        }
    }

    println!("\n{} Component installation complete! 🚀", "SUCCESS:".green().bold());
    Ok(())
}
