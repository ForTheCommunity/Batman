use crate::database_utils::get_connection::get_a_database_connection;
use rusqlite::{Connection, Error, Result};
use tabled::{
    self,
    settings::{object::Rows, style::BorderColor, Color, Padding, Style},
    Table, Tabled,
};

pub fn all_environment_variables() {
    match fetch_all_env_vars() {
        Ok(all) => {
            let mut table: Table = Table::new(all);

            // Customizing Borders.
            table.with(Style::modern_rounded());

            table.with(BorderColor::new().set_left(Color::FG_RED));
            table.with(BorderColor::new().set_right(Color::FG_RED));
            table.with(BorderColor::new().set_top(Color::FG_RED));
            table.with(BorderColor::new().set_bottom(Color::FG_RED));

            table.modify(Rows::new(..), Padding::new(3, 3, 0, 0));
            println!("{}", table);
        }
        Err(error) => println!("Error -> {}", error),
    }
}

fn fetch_all_env_vars() -> Result<Vec<EnvironmentVariableData>, Error> {
    let conn: Connection = get_a_database_connection();

    let mut sql_statement =
        conn.prepare("SELECT env_id, env_name,creation_date,modified_date FROM Env_Table")?;

    let environment_variables_data: Result<Vec<EnvironmentVariableData>> = sql_statement
        .query_map([], |row| {
            Ok(EnvironmentVariableData {
                env_id: row.get(0)?,
                env_name: row.get(1)?,
                creation_date: row.get(2)?,
                modified_date: row.get(3)?,
            })
        })?
        .collect();
    environment_variables_data
}

#[derive(Tabled)]
struct EnvironmentVariableData {
    env_id: i32,
    env_name: String,
    creation_date: String,
    modified_date: String,
}
