use std::path::{Path, PathBuf};
use anyhow::Result;

// find the root of the nextjs project..
pub fn find_project_root() -> Option<PathBuf> { 
    //Result -> Option -> (?) early-err
    let current_dir = std::env::current_dir().ok()?;
    //PathBuf -> &Path
    let mut current = current_dir.as_path();

    loop {
        //cheking for package.json
        let package_json = current.join("package.json");
        if package_json.exists() {
            return Some(current.to_path_buf());
        }

        //move up in directo..
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            return None;
        }
    }
}


//CLI rootdir
pub fn get_cli_root() -> PathBuf {
    let exe_path = std::env::current_exe().expect("Failed to get executable path");
    let cli_root = exe_path.parent().unwrap().to_path_buf();
    cli_root
}


// ensure dir exist else create
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}


pub fn get_package_manager(project_root: &Path) -> &'static str {
    if project_root.join("deno.json").exists() || project_root.join("deno.jsonc").exists() {
        return "deno";
    }
    if project_root.join("bun.lockb").exists() {
        return "bun";
    }
    if project_root.join("pnpm-lock.yaml").exists() {
        return "pnpm";
    }
    if project_root.join("yarn.lock").exists() {
        return "yarn";
    }
    if project_root.join("package-lock.json").exists() {
        return "npm";
    }

    "npm"
}



pub fn get_install_command(package_manager: &str) -> &'static str {
    match package_manager {
        "yarn" => "add",
        "bun" => "add",
        "deno" => "add",
        "npm" => "install",
        "pnpm" => "install",
        _ => "install",
    }
}
