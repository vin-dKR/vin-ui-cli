use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use std::fs;

pub fn get_templates_dir() -> PathBuf {
    // Try several locations in order of preference:
    
    // 1. Check environment variable (useful for development & testing)
    if let Ok(template_path) = std::env::var("VIN_UI_TEMPLATES") {
        let path = PathBuf::from(template_path);
        if path.exists() && path.is_dir() {
            return path;
        }
    }
    
    // 2. Check for templates next to the binary
    if let Ok(exe_path) = std::env::current_exe() {
        let binary_adjacent = exe_path.parent().unwrap().join("templates");
        if binary_adjacent.exists() && binary_adjacent.is_dir() {
            return binary_adjacent;
        }
    }
    
    // 3. Check user's config directory
    if let Some(config_dir) = dirs::config_dir() {
        let config_templates = config_dir.join("vin-ui").join("templates");
        if config_templates.exists() && config_templates.is_dir() {
            return config_templates;
        }
    }
    
    // 4. Check current directory and parents (for development)
    if let Ok(current_dir) = std::env::current_dir() {
        let mut dir = current_dir;
        loop {
            let templates_dir = dir.join("templates");
            if templates_dir.exists() && templates_dir.is_dir() {
                return templates_dir;
            }
            
            // Move up one directory
            if let Some(parent) = dir.parent() {
                dir = parent.to_path_buf();
            } else {
                break;
            }
        }
    }
    
    // 5. If couldn't find templates, return a default location
    // This will typically cause a "templates not found" error later
    if let Some(config_dir) = dirs::config_dir() {
        config_dir.join("vin-ui").join("templates")
    } else {
        PathBuf::from("./templates")
    }
}

// Install templates to the user's config directory
pub fn install_templates() -> Result<PathBuf> {
    // First try to find development templates
    let dev_templates = find_dev_templates();
    
    // Get the user's config directory
    let config_dir = dirs::config_dir()
        .context("Could not find user config directory")?
        .join("vin-ui");
    
    let templates_dest = config_dir.join("templates");
    
    // Only install if we found development templates and the destination doesn't exist
    if let Some(source_dir) = dev_templates {
        if !templates_dest.exists() {
            // Create the destination directory
            fs::create_dir_all(&templates_dest)
                .context("Failed to create templates directory")?;
            
            // Copy all templates
            copy_dir_recursively(&source_dir, &templates_dest)
                .context("Failed to copy templates")?;
            
            println!("Installed templates to {}", templates_dest.display());
        }
    }
    
    Ok(templates_dest)
}

// Find templates in development environment
fn find_dev_templates() -> Option<PathBuf> {
    // Try current directory and parents
    if let Ok(current_dir) = std::env::current_dir() {
        let mut dir = current_dir;
        loop {
            let templates_dir = dir.join("templates");
            if templates_dir.exists() && templates_dir.is_dir() {
                return Some(templates_dir);
            }
            
            if let Some(parent) = dir.parent() {
                dir = parent.to_path_buf();
            } else {
                break;
            }
        }
    }
    
    None
}

// Helper to copy directories recursively
fn copy_dir_recursively(source: &Path, destination: &Path) -> Result<()> {
    // Create the destination directory if it doesn't exist
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }
    
    // Read source directory entries
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = destination.join(path.file_name().unwrap());
        
        if path.is_dir() {
            // Recursively copy subdirectories
            copy_dir_recursively(&path, &dest_path)?;
        } else {
            // Copy files
            fs::copy(&path, &dest_path)?;
        }
    }
    
    Ok(())
}
