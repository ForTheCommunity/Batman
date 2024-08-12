use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a New Environment Variable..
    Create {
        /// Environment Variable (Name)
        #[arg(short = 'n', long)]
        env_name: String,
        /// Environment Variable (Key)
        #[arg(short = 'k', long)]
        env_key: String,
        /// Environment Variable (Value)
        #[arg(short = 'v', long)]
        env_value: String,
    },
    /// Delete Environment Variable..
    Delete {
        /// Environment Variable (Name)
        #[arg(short = 'n', long)]
        env_name: String,
    },
    /// View All Environment Variables..
    View,
    /// Update Environment Variable.
    Update,
    /// Initialize a Environment Variable
    Init {
        /// Name of Environment Variable.
        #[arg(short = 'n', long)]
        env_name: String,
    },
}
