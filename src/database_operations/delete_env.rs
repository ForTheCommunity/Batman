use colored::Colorize;
use rusqlite::{params, Connection, Error, Result};

use crate::database_utils::get_connection::get_a_database_connection;

pub fn delete_env(env_name: String) {
    match delete_env_from_database(env_name) {
        Ok(usize_value) => {
            println!(
                "{}...{}",
                "Environment Variable Deleted..".cyan(),
                usize_value
            );
        }

        Err(error) => {
            println!(
                "{}...{}",
                "Error Occurred While Deleting Environment Variable From Database..".red(),
                error
            );
        }
    }
}

fn delete_env_from_database(env_name: String) -> Result<usize, Error> {
    let conn: Connection = get_a_database_connection();

    let delete_status: usize = conn.execute(
        "
        DELETE FROM Env_Table WHERE env_name = ?
        ",
        params![env_name],
    )?;

    Ok(delete_status)
}
