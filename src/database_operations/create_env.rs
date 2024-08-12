use crate::database_utils::get_connection::get_a_database_connection;
use chrono::{self, DateTime, Datelike, Local};
use rusqlite::{Connection, Error, Result};

pub fn create_new_env(env_name: String, env_key: String, env_value: String) {
    match insert_new_env_variable(env_name, env_key, env_value) {
        Ok(usize_value) => {
            println!("New Environment Variable Stored.. {}", usize_value);
        }
        Err(error) => {
            println!(
                "Error Occurred While Creating a New Environment Variable... ERROR -> {}",
                error
            );
        }
    }
}

fn insert_new_env_variable(
    env_name: String,
    env_key: String,
    env_value: String,
) -> Result<usize, Error> {
    let conn: Connection = get_a_database_connection();

    let local_date: DateTime<Local> = Local::now();
    let year: i32 = local_date.year();
    let month: u32 = local_date.month();
    let day: u32 = local_date.day();

    let date_now: String = format!("{:04} - {:02} - {:02}", year, month, day);
    let creation_date = date_now.clone();
    let modified_date = date_now;

    let new_env_var_insertion_status :usize = conn.execute("
        INSERT INTO Env_Table (env_name,env_key,env_value,creation_date,modified_date) VALUES (?1,?2,?3,?4,?5) 
        ", (env_name,env_key,env_value,creation_date,modified_date),
    )?;

    Ok(new_env_var_insertion_status)
}
