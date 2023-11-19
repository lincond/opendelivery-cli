use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    base_url: String,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Auth {
        #[arg(long)]
        client_id: String,
        #[arg(long)]
        client_secret: String
    },
}

fn auth(client_id: &String, client_secret: &String) {
    println!("client_id: {client_id}");
    println!("client_secret: {client_secret}");
}
    

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Auth { client_id, client_secret }) => {
            auth(client_id, client_secret)
        }
        None => {}
    };
}
