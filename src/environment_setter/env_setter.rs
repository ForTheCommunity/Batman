use colored::Colorize;
use rusqlite::{params, Connection, Error, Statement};

use crate::{
    database_utils::get_connection::get_a_database_connection,
    environment_setter::generate_env_var_shell_script::generate_script,
};

pub fn env_setter(env_name: String) {
    match check_env_var_is_valid_or_not(&env_name) {
        Ok(true) => {
            println!(
                "{}",
                "Environment Variable Found in Database."
                    .bright_blue()
                    .on_bright_white()
            );
            println!(
                "{} : {}",
                "Setting Up Environment For Environment Variable"
                    .bright_white()
                    .on_black(),
                &env_name.bright_white().on_bright_black()
            );

            let env_key_value = fetch_env_data(&env_name).unwrap();
            let _ = generate_script(env_key_value);
        }
        Ok(false) => {
            println!(
                "{}",
                "Environment Variable Not Found in Database."
                    .bright_red()
                    .on_bright_white()
            );
        }
        Err(error) => {
            println!("Error Occurred.. ERROR -> {}", error);
        }
    }
}

fn check_env_var_is_valid_or_not(env_var_name: &str) -> Result<bool, Error> {
    let conn: Connection = get_a_database_connection();

    // println!("Checking for : {}", old_env_var_name);

    let mut sql_statement: Statement = conn.prepare(
        "
        SELECT COUNT(*) FROM Env_Table WHERE env_name = ?1
        ",
    )?;

    // Check if any rows are returned
    let trimmed_env_name: &str = env_var_name.trim();
    let count: i64 = sql_statement.query_row(&[trimmed_env_name], |row| row.get(0))?;

    // println!("ENV_COUNT ===>>> {}",count);

    if count == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn fetch_env_data(env_var_name: &str) -> Result<EnvKeyValue, Error> {
    let conn: Connection = get_a_database_connection();

    // sql_statement
    let mut sql_statement: Statement = conn.prepare(
        "
            SELECT env_name, env_key, env_value
            FROM Env_Table
            WHERE env_name = ?
        ",
    )?;

    // Executing Statement With Query Parameter
    let mut rows = sql_statement.query(params![env_var_name])?;

    let mut env_name: String = String::new();
    let mut env_key: String = String::new();
    let mut env_value: String = String::new();

    while let Some(row) = rows.next()? {
        env_name = row.get(0)?;
        env_key = row.get(1)?;
        env_value = row.get(2)?;
    }

    Ok(EnvKeyValue {
        env_name,
        env_key,
        env_value,
    })
}

pub struct EnvKeyValue {
    pub env_name: String,
    pub env_key: String,
    pub env_value: String,
}
