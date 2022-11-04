use serde::{Deserialize, Serialize};

/// Effects that may affect Players or other entities
#[derive(Debug, Serialize, Deserialize)]
pub enum Effects {
    Poisoned,
    Sick,
}