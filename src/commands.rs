use crate::cli::Commands;

pub fn execute_command(command: Commands) {
    match command {
        Commands::Test { list } => {
            if list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
    }
}
