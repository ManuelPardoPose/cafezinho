use crate::{
    file_io::file_utils,
    models::models::{CoffeeType, ConsumptionEntry},
};
use rusqlite::{params, Connection, Error};

pub const DB_NAME: &str = "data.db3";

pub fn get_conn() -> Connection {
    let mut path = file_utils::DATA_FOLDER_PATH.to_owned();
    path.push_str(DB_NAME);

    return Connection::open(path).expect("SQLite Error");
}

pub fn setup_db() -> Result<(), Error> {
    let conn = get_conn();
    setup_coffee_types(&conn)?;
    setup_consumption_entry(&conn)?;
    Ok(())
}

pub fn setup_consumption_entry(conn: &Connection) -> Result<(), Error> {
    let _ = conn.execute("DROP TABLE consumption_entry", ());

    conn.execute(
        "CREATE TABLE consumption_entry (
id    INTEGER PRIMARY KEY,
coffee_type_id    INTEGER,
time    TEXT NOT NULL,
FOREIGN KEY(coffee_type_id) REFERENCES coffee_type(id)
)",
        (),
    )?;
    let dummy_entries = vec![
        ConsumptionEntry::new(1, 1),
        ConsumptionEntry::new(2, 1),
        ConsumptionEntry::new(3, 4),
        ConsumptionEntry::new(4, 4),
        ConsumptionEntry::new(5, 4),
        ConsumptionEntry::new(6, 2),
    ];
    for entry in dummy_entries {
        conn.execute(
            "INSERT INTO consumption_entry (coffee_type_id, time) VALUES (?1, ?2)",
            params![&entry.coffee_type_id, &entry.time],
        )?;
    }
    Ok(())
}

pub fn setup_coffee_types(conn: &Connection) -> Result<(), Error> {
    let _ = conn.execute("DROP TABLE coffee_type", ());

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

    stmt = conn.prepare("SELECT id, coffee_type_id, time FROM consumption_entry")?;
    let consumption_entry_iter = stmt.query_map([], |row| {
        Ok(ConsumptionEntry::from_date(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    })?;

    for consumption_entry in consumption_entry_iter {
        println!("{:?}", consumption_entry.unwrap());
    }

    //JOINED:

    // stmt = conn.prepare(
    //     "SELECT consumption_entry.id, coffee_type.name
    // FROM consumption_entry
    // INNER JOIN coffee_type
    // ON coffee_type.id = consumption_entry.coffee_type_id"
    // )?;
    // let consumption_entry_iter = stmt.query_map([], |row| {
    //     let entry_id: i32 = row.get(0)?;
    //     let coffee_type_name: String = row.get(1)?;
    //     Ok(format!("{}, {}", entry_id, coffee_type_name))
    // })?;

    // for consumption_entry in consumption_entry_iter {
    //     println!("JOINED: {}", consumption_entry.unwrap());
    // }
    Ok(())
}

pub fn add_entry(coffee_type_str: String) -> Result<(), Error> {
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT id, name FROM coffee_type WHERE name=:name")?;
    let mut coffee_type_iter = stmt.query_map(&[(":name", &coffee_type_str)], |row| {
        Ok(CoffeeType {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    let first = coffee_type_iter.next();
    if first.is_none() {
        // TODO: add coffee type
    }

    let coffee_type = first.unwrap().unwrap();
    let new_entry = ConsumptionEntry::new(0, coffee_type.id); // id is irrelevant here
    conn.execute(
        "INSERT INTO consumption_entry (coffee_type_id, time) VALUES (?1, ?2)",
        params![&new_entry.coffee_type_id, &new_entry.time],
    )?;
    Ok(())
}
