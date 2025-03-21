use crate::turn::TurnResult;

/// Calcule le score pour un objectif donné.
///
/// # Arguments
///
/// * `target` - La valeur cible de l’objectif.
/// * `result` - Le résultat obtenu pour cet objectif (compteur et nombre de "miss").
/// * `force` - La force du joueur.
///
/// # Retour
///
/// Le score calculé pour cet objectif.
pub fn calculate_score(target: u8, result: &TurnResult, force: u32) -> u32 {
    let counter = result.counter;
    let miss = result.miss;
    // Calcul de la différence brute
    let diff_raw = if target >= counter {
        target - counter
    } else {
        counter - target
    };
    // Calcul de la différence circulaire
    let diff = if diff_raw > 50 { 100 - diff_raw } else { diff_raw };

    // Détermination de la base selon la différence
    let base = if diff == 0 {
        100
    } else if diff <= 5 {
        80
    } else if diff <= 10 {
        60
    } else if diff <= 20 {
        40
    } else {
        20
    };

    // Calcul du score
    (base as u32 + force) / (miss + 1)
}
