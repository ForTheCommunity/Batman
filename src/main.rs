use clap::Parser;
use colored::Colorize;

mod cli;
use cli::cli_util::{Cli, Commands};

mod database_utils;

use database_utils::{
    config_folder_setup::setup_config_folder, initial_database_setup::initial_db_setup,
};

mod database_operations;
use database_operations::{
    create_env::create_new_env, delete_env::delete_env, show_envs::all_environment_variables,
};
use environment_setter::env_setter::env_setter;

use crate::database_operations::update_env::update_env_var;

mod environment_setter;

fn main() {
    let _ = setup_config_folder();
    let _ = initial_db_setup();

    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create {
            env_name,
            env_key,
            env_value,
        }) => {
            create_new_env(
                env_name.to_owned(),
                env_key.to_owned(),
                env_value.to_owned(),
            );
        }

        Some(Commands::Delete { env_name }) => {
            println!("Deleting Environment Variable... {}", env_name);
            delete_env(env_name.to_owned());
        }

        Some(Commands::View) => {
            println!("{}", "Showing All Environment Variables.".cyan());
            all_environment_variables();
        }

        Some(Commands::Update) => {
            println!("{}", "Updating Environment Variable.....".blue().on_white());
            update_env_var();
        }

        Some(Commands::Init { env_name }) => {
            env_setter(env_name.to_owned());
        }

        None => {
            println!("Please Provide Some Data To Continue..");
        }
    }
}
