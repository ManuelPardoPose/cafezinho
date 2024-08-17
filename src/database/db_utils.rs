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

    let _ = conn.execute(
        "DROP TABLE coffee_type",
        (),
    );

    conn.execute(
        "CREATE TABLE coffee_type (
id    INTEGER PRIMARY KEY,
name  TEXT NOT NULL
)",
        (),
    )?;
    let coffee_types = vec![
        CoffeeType::new(1, "Espresso Solo".to_string()),
        CoffeeType::new(2, "Espresso Doppio".to_string()),
        CoffeeType::new(3, "Lungo".to_string()),
        CoffeeType::new(4, "Americano".to_string()),
        CoffeeType::new(5, "Cappuccino".to_string()),
        CoffeeType::new(6, "Latte Macchiato".to_string()),
        CoffeeType::new(7, "Espresso Macchiato".to_string()),
        CoffeeType::new(8, "Affogato al caffè".to_string()),
        CoffeeType::new(9, "Café au lait".to_string()),
    ];
    for coffee_type in coffee_types {
        conn.execute(
            "INSERT INTO coffee_type (name) VALUES (?1)",
            params![&coffee_type.name],
        )?;
    }
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
