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

pub fn get_coffee_type(coffee_type_str: &str) -> Option<CoffeeType> {
    let conn = get_conn();
    let mut stmt = match conn.prepare("SELECT id, name FROM coffee_type WHERE name=:name") {
        Err(_) => return None,
        Ok(stmt) => stmt,
    };
    let mut coffee_type_iter = match stmt.query_map(&[(":name", &coffee_type_str)], |row| {
        Ok(CoffeeType {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }) {
        Err(_) => return None,
        Ok(iter) => iter,
    };
    let first_coffee_type = match coffee_type_iter.next() {
        None => return None,
        Some(first_result) => match first_result {
            Err(_) => return None,
            Ok(coffee_type) => coffee_type,
        },
    };
    return Some(first_coffee_type);
}

pub fn add_coffee_type(coffee_type_str: &str) -> Option<CoffeeType> {
    let conn = get_conn();
    let new_coffee_type = CoffeeType::new(1, coffee_type_str.to_string());
    match conn.execute(
        "INSERT INTO coffee_type (name) VALUES (?1)",
        params![&new_coffee_type.name],
    ) {
        Err(_) => return None,
        Ok(_) => {}
    };
    return get_coffee_type(coffee_type_str); // tries to retrieve from db to confirm existence
}

pub fn add_entry(coffee_type_str: String) -> Result<(), &'static str> {
    let conn = get_conn();
    let coffee_type = match get_coffee_type(&coffee_type_str) {
        None => match add_coffee_type(&coffee_type_str) {
            None => return Err("CoffeeType could not be added"),
            Some(v) => v,
        },
        Some(d) => d,
    };

    let new_entry = ConsumptionEntry::new(0, coffee_type.id); // id is irrelevant here
    match conn.execute(
        "INSERT INTO consumption_entry (coffee_type_id, time) VALUES (?1, ?2)",
        params![&new_entry.coffee_type_id, &new_entry.time],
    ) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("ConsumptionEntry could not be created"),
    }
}

pub fn print_stats() -> Result<(), Error> {
    println!("-------------------STATS-------------------");
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM consumption_entry")?;
    let mut entry_iter = stmt.query_map([], |row| {
        let count: i32 = row.get(0)?;
        Ok(format!("{}", count))
    })?;
    let total_coffees_consumed = match entry_iter.next() {
        Some(value) => value,
        None => return Ok(()),
    };
    let total_coffees_consumed = total_coffees_consumed.unwrap_or_default();

    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM consumption_entry
        WHERE date(time) = date()
        ",
    )?;
    let mut entry_iter = stmt.query_map([], |row| {
        let count: i32 = row.get(0)?;
        Ok(format!("{}", count))
    })?;
    let coffees_consumed_today = match entry_iter.next() {
        Some(value) => value,
        None => return Ok(()),
    };
    let coffees_consumed_today = coffees_consumed_today.unwrap_or_default();

    let mut stmt = conn.prepare(
        "SELECT coffee_type.name
        FROM consumption_entry
        INNER JOIN coffee_type
        ON coffee_type.id = consumption_entry.coffee_type_id
        GROUP BY coffee_type.name
        ORDER BY COUNT(coffee_type.name) DESC",
    )?;
    let mut entry_iter = stmt.query_map([], |row| {
        let coffee_type_name: String = row.get(0)?;
        Ok(format!("{}", coffee_type_name))
    })?;
    let favorite_coffee_type = match entry_iter.next() {
        Some(value) => value,
        None => return Ok(()),
    };
    let favorite_coffee_type = favorite_coffee_type.unwrap_or_default();

    println!("consumed in total: {}", total_coffees_consumed);
    println!("consumed today: {}", coffees_consumed_today);
    println!("favorite coffee type: {}", favorite_coffee_type);
    return Ok(());
}
