/// Trait to model a weapon
pub trait Weapon {
    fn new(name: String) -> Self;
    fn get_name(&self) -> &str;
    fn get_attack(&self) -> f64;
}
