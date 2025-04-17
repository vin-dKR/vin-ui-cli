use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use serde_json;

use crate::models::component::ComponentConfig;

pub fn load_component_config(config_path: &Path) -> Result<Option<ComponentConfig>> {
    if !config_path.exists() {
        return Ok(None);
    }

    let config_content = fs::read_to_string(config_path)
        .context(format!("Failed to read config file: {}", config_path.display()))?;

    let config: ComponentConfig = serde_json::from_str(&config_content)
        .context(format!("Failed to parse config file: {}", config_path.display()))?;

    Ok(Some(config))
}


// Lists all available components in the templates directory
pub fn get_available_components(template_dir: &Path) -> Result<Vec<String>> {
    if !template_dir.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(template_dir)?;
    let mut components = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "tsx") {
                if let Some(stem) = path.file_stem(){
                    if let Some(name) = stem.to_str() {
                        components.push(name.to_string());
                    }
                }
            }
        }
    }
    
    components.sort();
    Ok(components)
}


// Adds a utility function to the utils.ts file
pub fn add_utility(lib_dir: &Path, util_name: &str, template_dir: &Path) -> Result<()> {
    let utils_file = lib_dir.join("utils.ts");
    let mut utils_content = String::new();

    if utils_file.exists() {
        utils_content = fs::read_to_string(&utils_file)?;
    }


    let util_template = template_dir.join("utils").join(format!("{}.ts", util_name));
    if util_template.exists() {
        let util_content = fs::read_to_string(&util_template)?;

        if !utils_content.contains(&util_content) {
            if !utils_content.is_empty() {
                utils_content.push_str("\n\n");
            }

            utils_content.push_str(&util_content);
            fs::write(&utils_file, utils_content)?;

            return Ok(());
        }
    }
    Ok(())
}
