mod cli;
mod commands;
mod config;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        config::handle_config(config_path);
    }

    config::handle_debug(cli.debug);

    if let Some(command) = cli.command {
        commands::execute_command(command);
    }

    // Continued program logic goes here...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        cli::Cli::command().debug_assert()
    }
}
