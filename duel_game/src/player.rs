/// Structure représentant un joueur dans le jeu de duel.
#[derive(Debug, Clone)]
pub struct Player {
    /// Nom du joueur.
    pub name: String,
    /// Vitalité (points de vie) du joueur.
    pub vitality: u32,
    /// Vitesse du joueur, utilisée pour l'incrémentation du compteur.
    pub speed: u32,
    /// Force du joueur, qui intervient dans le calcul du score.
    pub strength: u32,
}

impl Player {
    /// Crée un nouveau joueur avec les valeurs spécifiées.
    pub fn new(name: String, vitality: u32, speed: u32, strength: u32) -> Self {
        Player { name, vitality, speed, strength }
    }
}

impl Default for Player {
    /// Valeurs par défaut pour un joueur :
    /// nom "Joueur", vitalité 50, vitesse 50, force 50.
    fn default() -> Self {
        Player {
            name: String::from("Joueur"),
            vitality: 50,
            speed: 50,
            strength: 50,
        }
    }
}
