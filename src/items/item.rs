use serde::{Deserialize, Serialize};

/// Models an Item with name and value
#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    name: String,
    value: f64,
}

impl Item {
    pub fn new(name: String, value: f64) -> Self {
        Item { name, value }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}
