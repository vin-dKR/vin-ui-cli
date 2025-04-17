use colored::*;
use dialoguer::{Confirm, Select};

pub fn print_success(msg: &str) {
    prinln!("{} {}", "SUCCESS:".green().bold(), msg);
}

pub fn print_error(msg: &str) {
    println!("{} {}", "ERROR:".red().bold(), msg);
}

pub fn print_warning(msg: &str) {
    println!("{} {}", "WARNING:".yellow().bold(), msg);
}

pub fn print_info(msg: &str) {
    println!("{} {}", "INFO".blue().bold(), msg);
}

pub fn confirm(prompt: &str, default: bool) -> bool {
    Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()
        .unwrap_or(false)
} 
