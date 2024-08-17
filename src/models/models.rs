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
}

impl ConsumptionEntry {
    pub fn new(id: i32, coffee_type_id: i32) -> Self {
        Self { id, coffee_type_id }
    }
}
