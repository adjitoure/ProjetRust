use rand::Rng;

/// Génère une liste d’objectifs aléatoires.
/// Chaque objectif est un nombre entre 0 et 100.
///
/// # Arguments
///
/// * `count` - Le nombre d’objectifs à générer.
///
/// # Retour
///
/// Un vecteur contenant `count` nombres aléatoires.

pub fn generate_objectives(count: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(0..=100)).collect()
}
