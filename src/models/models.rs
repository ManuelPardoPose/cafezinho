use std::str::FromStr;

use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct CoffeeType {
    pub id: i32,
    pub name: String,
}

impl CoffeeType {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug)]
pub struct ConsumptionEntry {
    pub id: i32,
    pub coffee_type_id: i32,
    pub time: String,
}

impl ConsumptionEntry {
    pub fn new(id: i32, coffee_type_id: i32) -> Self {
        Self::from_date(id, coffee_type_id, Local::now().to_string())
    }

    pub fn from_date(id: i32, coffee_type_id: i32, time: String) -> Self {
        Self {
            id,
            coffee_type_id,
            time,
        }
    }
}

fn time_string_to_date(time_string: &String) -> DateTime<Local> {
    return DateTime::from_str(time_string).unwrap_or_default();
}
