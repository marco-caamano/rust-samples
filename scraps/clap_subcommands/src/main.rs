use clap::{Parser, Subcommand};

/// Command-line tool for managing users and hosts
#[derive(Parser, Debug)]
#[command(name = "cli_tool", about = "CLI tool for user and host management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage users
    User {
        #[command(subcommand)]
        subcommand: UserCommands,
    },

    /// Manage hosts
    Hosts {
        #[command(subcommand)]
        subcommand: HostsCommands,
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

#[derive(Subcommand, Debug)]
enum HostsCommands {
    /// Enable a host
    Enable {
        /// The name of the host to enable
        hostname: String,
    },
    /// Disable a host
    Disable {
        /// The name of the host to disable
        hostname: String,
    },
    /// Get the status of a host
    Status {
        /// The name of the host to check the status of
        hostname: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::User { subcommand } => match subcommand {
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
        },
        Commands::Hosts { subcommand } => match subcommand {
            HostsCommands::Enable { hostname } => {
                println!("Enabling host: {}", hostname);
            }
            HostsCommands::Disable { hostname } => {
                println!("Disabling host: {}", hostname);
            }
            HostsCommands::Status { hostname } => {
                println!("Getting status for host: {}", hostname);
            }
        },
    }
}
