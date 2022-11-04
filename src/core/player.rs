use serde::{Deserialize, Serialize};

use super::effects::Effects;

/// A struct representing the player of the story
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    /// The name of the player
    pub name: String,
    /// The Health Points 
    pub hp: f64,
    /// The Mana Points
    pub mp: f64,
    /// A vector for any applied / active effects
    pub status_effects: Vec<Effects>,
}

