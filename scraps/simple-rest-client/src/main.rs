use clap::{Parser, Subcommand};

/// Simple REST Client
#[derive(Parser, Debug)]
#[command(
    name = "simple_rest_client",
    about = "CLI for simple user management on REST server"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage users
    Users {
        #[command(subcommand)]
        subcommand: UserCommands,
    },
}

#[derive(Subcommand, Debug)]
enum UserCommands {
    /// List all users
    List,
    /// Add a new user
    Add {
        /// The ID of the new user
        #[arg(short, long)]
        id: u32,

        /// The name of the new user
        #[arg(short, long)]
        name: String,
    },
    /// Update an existing user
    Update {
        /// The ID of the user to update
        id: u32,

        /// The name of the user to update
        name: String,
    },
    /// Delete a user
    Delete {
        /// The ID of user to delete
        id: u32,
    },
}

fn get_users() {
    //let body = reqwest::blocking::get("http://127.0.0.1:8080/users");
}

fn parse_user_command(subcommand: &UserCommands) {
    match subcommand {
        UserCommands::List => {
            println!("Listing all users...");
        }
        UserCommands::Add { id, name } => {
            println!("Adding id:{} user: {}", id, name);
        }
        UserCommands::Update { id, name } => {
            println!("Updating id:{} user: {}", id, name);
        }
        UserCommands::Delete { id } => {
            println!("Deleting user: {}", id);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Users { subcommand } => {
            parse_user_command(subcommand);
        }
    }
}
