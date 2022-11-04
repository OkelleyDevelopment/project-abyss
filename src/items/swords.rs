use super::traits::Weapon;

/// Simple Example weapon
pub struct Sword {
    /// The name of the sword
    name: String,
    /// The attack damage of the sword
    attack: f64,
}

impl Weapon for Sword {
    fn new(name: String) -> Self {
        Sword {
            name,
            attack: 10 as f64,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_attack(&self) -> f64 {
        self.attack
    }
}
