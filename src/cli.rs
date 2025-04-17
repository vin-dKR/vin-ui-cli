use clap::{Arg, Command};
use colored::*;

pub fn build_cli() -> Command {
    Command::new("vin-ui")
        .version("1.0")
        .author("Vinod KR <vinodkumarmurmu62@gmail.com>")
        .about("A CLI UI library that is independent of package managers")
        .subcommand(
            Command::new("add")
                .about("Add a component to your Next.js project.")
                .arg(
                    Arg::new("COMPONENT_NAME")
                        .help("The component to add.")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List all available components."),
        )
        .subcommand(
            Command::new("init")
                .about("Initialize the UI components directory.")
                .arg(
                    Arg::new("help")
                        .long("help")
                        .short('h') // short should be a single char
                        .help("Print help information")
                        .action(clap::ArgAction::Help), // tell clap it's the built-in help
                ),
        )
}


pub fn print_help() {
    println!("{}",
        "
█░█ █ █▄░█ ▄▄ █░█ █
▀▄▀ █ █░▀█ ░░ █▄█ █
".bright_blue()
    );
    println!("{}", "A powerful UI component manager for Next.js projects".bright_white());
    println!();
    println!("{}", "USAGES:".yellow());
    println!("    vin-ui [SUBCOMMAND]");
    println!();
    println!("{}","SUBCOMMAND".yellow());
    println!("    add <COMPONENT_NAME>        Add a component to your project");
    println!("    list                        List all available components");
    println!("    init                        Initialize the components directory");
    println!();
    println!("{}", "FLAGS: ".yellow());
    println!("    -h  --help                  Print this help message");
    println!();
    println!("{}", "EXAMPLES:".yellow());
    println!("    vin-ui add Button           # Add Button component");
    println!("    vin-ui list                 # List all available components");
    println!();
}
