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
