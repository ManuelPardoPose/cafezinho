use crate::{file_io::file_utils, models::models::CoffeeType};
use rusqlite::{params, Connection, Error};

pub const DB_NAME: &str = "data.db3";

pub fn get_conn() -> Connection {
    let mut path = file_utils::DATA_FOLDER_PATH.to_owned();
    path.push_str(DB_NAME);

    return Connection::open(
        path
    ).expect("SQLite Error");
}

pub fn setup_db() -> Result<(), Error> {
    let conn = get_conn();

    conn.execute(
        "CREATE TABLE coffee_type (
id    INTEGER PRIMARY KEY,
name  TEXT NOT NULL
)",
        (),
    )?;
    let type1 = CoffeeType::new(1, "Espresso".to_string());
    conn.execute(
        "INSERT INTO coffee_type (name) VALUES (?1)",
        params![&type1.name],
    )?;
    Ok(())
}

pub fn print_tables() -> Result<(), Error> {
    let conn = get_conn();

    let mut stmt = conn.prepare("SELECT id, name FROM coffee_type")?;
    let coffee_type_iter = stmt.query_map([], |row| {
        Ok(CoffeeType {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for coffee_type in coffee_type_iter {
        println!("{:?}", coffee_type.unwrap());
    }
    Ok(())
}
