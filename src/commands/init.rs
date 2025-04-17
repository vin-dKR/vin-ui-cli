use std::fs;
use anyhow::Result;
use colored::*;

use crate::utils::project::{find_project_root, ensure_dir};
use crate::utils::ui::{print_error, print_success, print_info};

pub fn init_components_directory() -> Result<()> {
    //find next.js root
    let project_root = match find_project_root() {
        Some(root) => root,
        None => {
            print_error("No Next.js project found in the current directory or its parents!");
            return Ok(());
        }
    };

    let components_dir = project_root.join("components");
    let ui_dir = components_dir.join("ui");

    if !components_dir.exists() {
        ensure_dir(&components_dir)?;
        print_success(&format!(
            "Created components directory at {}",
            components_dir.display()
        ));
    }

    if !ui_dir.exists() {
        ensure_dir(&ui_dir)?;
        print_success(&format!(
            "Created ui directory at {}",
            ui_dir.display()
        ));
    } else {
        print_info(&format!(
            "UI components directory already exists at {}",
            ui_dir.display()
        ));
    }


    // crate /lib if doz not exists
    let lib_dir = project_root.join("lib");
    if !lib_dir.exists() {
        let _ = ensure_dir(&lib_dir);

        //create a basic utils.ts
        let utils_file = lib_dir.join("utils.ts");
        let basic_utils = r#"import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * Combines class names with tailwind-merge
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
"#;
        fs::write(&utils_file, basic_utils)?;

        print_success(&format!(
            "Created lib directory with basic utils.ts at {}",
            lib_dir.display()
        ));
    }

    println!("\n{} Initialization complete! Your project is ready for UI components.", "SUCCESS:".green().bold());
    println!("Run {} to see available components.", "vin-ui list".bright_cyan());
    Ok(())
}
