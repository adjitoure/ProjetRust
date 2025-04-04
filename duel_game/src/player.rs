#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub vitality: u32,
    pub speed: u32,
    pub strength: u32,
}

impl Player {
    pub fn new(name: String, vitality: u32, speed: u32, strength: u32) -> Self {
        Self { name, vitality, speed, strength }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: String::from("Joueur"),
            vitality: 50,
            speed: 50,
            strength: 50,
        }
    }
}
