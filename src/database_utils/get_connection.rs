use std::path::PathBuf;

use rusqlite::{self, Connection};

pub fn get_a_database_connection() -> Connection {
    let home_directory: PathBuf = dirs::home_dir().expect("Unable to Locate Home Directory..");

    let config_directory: PathBuf = home_directory.join(".batman");
    let database_directory: PathBuf = config_directory.join("database");
    let database_file: PathBuf = database_directory.join("batman_database.sqlite");

    let conn: Connection =
        Connection::open(database_file).expect("Unable to Establish a Connection to database file");
    conn
}
