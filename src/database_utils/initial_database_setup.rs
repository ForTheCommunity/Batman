use rusqlite::{Connection, Error, Result};

use super::get_connection::get_a_database_connection;

pub fn initial_db_setup() {
    println!("Setting Up Database..");

    // Settting Up Database.......
    match create_env_table() {
        Ok(usize_value) => {
            println!("Database is Ready..... {}", usize_value);
        }
        Err(error) => {
            println!(
                "Error Occurred While Setting Up Table (Database).. ERROR -> {}",
                error
            );
        }
    }
}

fn create_env_table() -> Result<usize, Error> {
    let conn: Connection = get_a_database_connection();

    let table_creation_status: usize = conn.execute(
        "
        CREATE TABLE IF NOT EXISTS Env_Table (
        env_id          INTEGER PRIMARY KEY ,
        env_name        VARCHAR(50) UNIQUE NOT NULL,
        env_key         VARCHAR(50)  NOT NULL,
        env_value       VARCHAR(100)  NOT NULL,
        creation_date   VARCHAR(50)  NOT NULL,
        modified_date   VARCHAR(50)  NOT NULL
        )
        ",
        [],
    )?;

    Ok(table_creation_status)
}
