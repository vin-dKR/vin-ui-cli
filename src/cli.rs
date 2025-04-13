use clap::{App, Arg, SubCommand};
use colored::*;

pub fn build_cli() -> App<'static', 'static'> {
    App.new("vin-ui")
        .version("1.0")
        .author("Vinod KR <vinodkumarmurmu62@gmail.com>")
        .about("A cli UI Library that is independ of package managers")
        .subcommand(
            SubCommand::with_name("add")
                .about("add a component in your next js project.")
                .arg(
                    Arg::with_name("COMPONENT_NAME")
                        .help("The component to add.")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("list").about("List all available components.")
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize the UI components directory.")
                .arg(
                    Arg::with_name("help")
                        .long("help")
                        .short("help")
                        .help("Print help information"),
                )
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
