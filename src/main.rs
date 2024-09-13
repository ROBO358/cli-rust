use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, flatten_help = true)]
struct Args {
    #[command(subcommand)]
    sub: Command,
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    Hoge {
        fuga: String,
    }
}

fn print(fuga: &str) {
    println!("fuga: {:?}", fuga)
}


fn main() {
    match Args::parse().sub {
        Command::Hoge { fuga } => print(&fuga),
    }
}
