use chrono::{self, DateTime, Datelike, Local, Timelike};
use colored::Colorize;
use rusqlite::{params, Connection, Error, Statement};
use std::io::{self, Write};

use crate::database_utils::get_connection::get_a_database_connection;

use super::show_envs::all_environment_variables;

pub fn update_env_var() {
    println!(
        "{}",
        "Showing All Available Environment Variables"
            .white()
            .on_black()
    );
    all_environment_variables();
    let new_data: Result<UpdateEnvData, String> = read_new_env_data();

    match new_data {
        Ok(new_updated_data) => {
            let new_env_data_update_status: Result<usize, Error> = update_data(new_updated_data);

            match new_env_data_update_status {
                Ok(status) => {
                    println!(
                        "{} , {status}",
                        "Environment Variable Updated Succesfully"
                            .bright_green()
                            .on_black()
                    );
                }
                Err(error) => {
                    println!(
                        "{} , Error -> {error}",
                        "Error Occurred While Update Environment Variable"
                            .bright_red()
                            .on_black()
                    );
                }
            }
        }
        Err(error) => {
            println!("Error -> {}", error);
        }
    }
}

fn read_new_env_data() -> Result<UpdateEnvData, String> {
    // Read old_env_var_name
    let mut old_env_var_name: String = String::new();
    print!("    Old Environment Variable Name : ");
    io::stdout().flush().unwrap(); // Ensure the prompt is shown before input
    io::stdin()
        .read_line(&mut old_env_var_name)
        .expect("Failed to read line");

    let old_env_var_name: String = old_env_var_name.trim().to_string();

    let old_env_var_status: Result<bool, Error> = check_env_is_available_or_not(&old_env_var_name);

    match old_env_var_status {
        Ok(true) => {

            println!("  {}","( Enter Without Typing If You Want No Change.)".white().on_black());

            // read new_env_name
            let mut new_env_var_name: String = String::new();
            print!("    New Environment Variable Name : ");
            io::stdout().flush().unwrap(); // Ensure the prompt is shown before input
            io::stdin()
                .read_line(&mut new_env_var_name)
                .expect("Failed to read line");

            let new_env_var_name :Option<String> = {
              let trimmed = new_env_var_name.trim();
              if trimmed.is_empty(){
                None
              }else {
                  Some(trimmed.to_string())
              }
            };


            // read new_env_key
            let mut new_env_key: String = String::new();
            print!("    New Environment Variable Key : ");
            io::stdout().flush().unwrap(); // Ensure the prompt is shown before input
            io::stdin()
                .read_line(&mut new_env_key)
                .expect("Failed to read line");


          let new_env_key:Option<String> = {
              let trimmed = new_env_key.trim();
              if trimmed.is_empty(){
                None
              }else {
                  Some(trimmed.to_string())
              }
            };


            // read new_env_value
            let mut new_env_value: String = String::new();
            print!("    New Environment Variable Value : ");
            io::stdout().flush().unwrap(); // Ensure the prompt is shown before input
            io::stdin()
                .read_line(&mut new_env_value)
                .expect("Failed to read line");


          let new_env_value:Option<String> = {
              let trimmed = new_env_value.trim();
              if trimmed.is_empty(){
                None
              }else {
                  Some(trimmed.to_string())
              }
            };


            let updated_data: UpdateEnvData = UpdateEnvData {
                old_env_var_name,
                new_env_var_name,
                new_env_key,
                new_env_value,
            };
            Ok(updated_data)
        }

        Ok(false) => {
            Err("Environment Variable is not present in Database, Unable to Update Environment Variable Data.".to_string())
     }
     Err(error) => {
        Err(format!("Environment Variable is not present in Database , Unable to Update Environment Variable Data , Exact Error -> {}",error))
    }
}
}

fn check_env_is_available_or_not(old_env_var_name: &str) -> Result<bool, Error> {
    let conn: Connection = get_a_database_connection();

    // println!("Checking for : {}", old_env_var_name);

    let mut sql_statement: Statement = conn.prepare(
        "
        SELECT COUNT(*) FROM Env_Table WHERE env_name = ?1
        ",
    )?;

    // Check if any rows are returned
    let trimmed_old_env_name = old_env_var_name.trim();
    let count: i64 = sql_statement.query_row(&[trimmed_old_env_name], |row| row.get(0))?;

    // println!("ENV_COUNT ===>>> {}",count);

    if count == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn update_data(new_updated_data: UpdateEnvData) -> Result<usize, Error> {
    let conn: Connection = get_a_database_connection();

    let local_date: DateTime<Local> = Local::now();
    let year: i32 = local_date.year();
    let month: u32 = local_date.month();
    let day: u32 = local_date.day();
    let hour: u32 = local_date.hour();
    let minute: u32 = local_date.minute();
    let second: u32 = local_date.second();

    let date_now: String = format!(
        "{:04} - {:02} - {:02} , {:02} : {:02} : {:02}",
        year, month, day, hour, minute, second
    );
    let modified_date: String = date_now;

    let sql_query: usize = conn.execute(
        "
            UPDATE Env_Table
            SET env_name = COALESCE(?,env_name),
                env_key = COALESCE(?,env_key),
                env_value = COALESCE(?,env_value),
                modified_date = ?
            WHERE env_name = ?
        ",
        params![
            new_updated_data.new_env_var_name.as_deref(),
            new_updated_data.new_env_key.as_deref(),
            new_updated_data.new_env_value.as_deref(),
            modified_date,
            new_updated_data.old_env_var_name
        ],
    )?;

    Ok(sql_query)
}

struct UpdateEnvData {
    old_env_var_name: String,
    new_env_var_name: Option<String>,
    new_env_key: Option<String>,
    new_env_value: Option<String>,
}
