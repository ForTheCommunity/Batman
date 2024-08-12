use colored::*;
use dirs;
use std::fs;
use std::path::{Path, PathBuf};

pub fn setup_config_folder() {
    // Check Config Folder is Present or Not.
    println!("{}", "Checking Batman's Health.......".red());
    check_parent_directory_is_present_or_not();
}

fn check_parent_directory_is_present_or_not() {
    let home_directory: PathBuf = dirs::home_dir().expect("Unable to Locate Home Directory..");

    let config_directory: PathBuf = home_directory.join(".batman");

    let config_dir_exists_or_not: bool = Path::new(&config_directory).exists();

    if config_dir_exists_or_not {
        println!("Config Directory Does Exist.");
        check_database_directory_is_present_or_not();
    } else {
        println!("Config Directory Doesnot Exists..");
        create_config_directory();
    }
}

fn check_database_directory_is_present_or_not() {
    let home_directory: PathBuf = dirs::home_dir().expect("Unable to Locate Home Directory..");

    let config_directory: PathBuf = home_directory.join(".batman");
    let database_directory: PathBuf = config_directory.join("database");

    let database_dir_exists_or_not: bool = Path::new(&database_directory).exists();

    if database_dir_exists_or_not {
        println!("Database Directory Does Exists.");
        println!("Checking Database is Present or Not.");
        check_database_file_exists_or_not();
    } else {
        println!("Database Directory Doesnot Exists.");
        println!("Creating a New Database Directory and New Database File..");
        create_config_directory();
    }
}

fn create_config_directory() {
    let home_directory: PathBuf = dirs::home_dir().expect("Unable to Locate Home Directory..");

    let config_directory: PathBuf = home_directory.join(".batman/database");
    match fs::create_dir_all(config_directory) {
        Ok(_) => {
            println!("Created New Config Directory..");
            create_new_database_file();
        }
        Err(error) => println!(
            "Error Occurred While Creating New Config Directory , Error -> {}",
            error
        ),
    }
}

fn check_database_file_exists_or_not() {
    let home_directory: PathBuf = dirs::home_dir().expect("Unable to Locate Home Directory..");

    let config_directory: PathBuf = home_directory.join(".batman");
    let database_directory: PathBuf = config_directory.join("database");
    let database_file: PathBuf = database_directory.join("batman_database.sqlite");

    if database_file.exists() {
        println!("Database Already Exists , No need to create a New Database....");
    } else {
        create_new_database_file();
    }
}

fn create_new_database_file() {
    let home_directory: PathBuf = dirs::home_dir().expect("Unable to Locate Home Directory..");

    let config_directory: PathBuf = home_directory.join(".batman");
    let database_directory: PathBuf = config_directory.join("database");
    let database_file: PathBuf = database_directory.join("batman_database.sqlite");

    match fs::File::create(database_file) {
        Ok(_) => println!("New Database File Created."),
        Err(error) => println!(
            "Error Occurred while creating Database file.. , ERROR -> {}",
            error
        ),
    }
}
