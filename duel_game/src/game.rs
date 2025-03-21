use std::io::{self, Write};

use crate::objectives::generate_objectives;
use crate::score::calculate_score;
use crate::turn::play_objective;
use crate::player::Player;

/// Joue une manche complète entre deux joueurs.
///
/// Pour chaque joueur :
/// - On génère une série d’objectifs.
/// - Pour chaque objectif, on appelle la fonction `play_objective` qui gère le compteur.
/// - On calcule le score pour chaque objectif et on en déduit le score moyen.
/// 
/// Ensuite, on compare les scores moyens des deux joueurs :  
/// - Le joueur avec le score le plus élevé gagne la manche.
/// - Le perdant perd en vitalité la différence entre les scores.
/// - Le gagnant choisit un poison à appliquer au perdant : -5 de vitesse ou -5 de force.
/// 
/// Les statistiques des joueurs sont ensuite mises à jour.
pub fn play_round(player1: &mut Player, player2: &mut Player, nb_objectifs: u8) {
    println!("----- Nouvelle Manche -----");

    // Tour pour le joueur 1
    println!("\nAu tour de {} (Vitality: {}, Speed: {}, Strength: {})", player1.name, player1.vitality, player1.speed, player1.strength);
    let objectives1 = generate_objectives(nb_objectifs);
    println!("Objectifs: {:?}", objectives1);
    let mut total_score1: u32 = 0;
    for (i, target) in objectives1.iter().enumerate() {
        println!("\n--- {} - Objectif {}: Cible {} ---", player1.name, i + 1, target);
        let result = play_objective(player1.speed);
        let score_value = calculate_score(*target, &result, player1.strength);
        total_score1 += score_value;
        println!("Score pour cet objectif: {}", score_value);
    }
    let average_score1 = if nb_objectifs == 0 {
        0
    } else {
        (total_score1 as f32 / nb_objectifs as f32).ceil() as u32
    };
    println!("Score moyen de {}: {}", player1.name, average_score1);

    // Tour pour le joueur 2
    println!("\nAu tour de {} (Vitality: {}, Speed: {}, Strength: {})", player2.name, player2.vitality, player2.speed, player2.strength);
    let objectives2 = generate_objectives(nb_objectifs);
    println!("Objectifs: {:?}", objectives2);
    let mut total_score2: u32 = 0;
    for (i, target) in objectives2.iter().enumerate() {
        println!("\n--- {} - Objectif {}: Cible {} ---", player2.name, i + 1, target);
        let result = play_objective(player2.speed);
        let score_value = calculate_score(*target, &result, player2.strength);
        total_score2 += score_value;
        println!("Score pour cet objectif: {}", score_value);
    }
    let average_score2 = if nb_objectifs == 0 {
        0
    } else {
        (total_score2 as f32 / nb_objectifs as f32).ceil() as u32
    };
    println!("Score moyen de {}: {}", player2.name, average_score2);

    // Comparaison des scores et application des conséquences
    if average_score1 == average_score2 {
        println!("La manche est nulle, aucun changement de vitalité.");
    } else if average_score1 > average_score2 {
        let diff = average_score1 - average_score2;
        println!("{} gagne la manche !", player1.name);
        println!("{} perd {} points de vitalité.", player2.name, diff);
        if player2.vitality > diff {
            player2.vitality -= diff;
        } else {
            player2.vitality = 0;
        }
        // Choix du poison par le gagnant
        println!("{} choisissez un poison à appliquer à {} :", player1.name, player2.name);
        println!("1: -5 de speed");
        println!("2: -5 de strength");
        print!("Votre choix (1 ou 2): ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                if player2.speed >= 5 {
                    player2.speed -= 5;
                } else {
                    player2.speed = 0;
                }
                println!("{} subit -5 de speed.", player2.name);
            },
            "2" => {
                if player2.strength >= 5 {
                    player2.strength -= 5;
                } else {
                    player2.strength = 0;
                }
                println!("{} subit -5 de strength.", player2.name);
            },
            _ => println!("Choix invalide. Aucun poison appliqué."),
        }
    } else { // average_score2 > average_score1
        let diff = average_score2 - average_score1;
        println!("{} gagne la manche !", player2.name);
        println!("{} perd {} points de vitalité.", player1.name, diff);
        if player1.vitality > diff {
            player1.vitality -= diff;
        } else {
            player1.vitality = 0;
        }
        println!("{} choisissez un poison à appliquer à {} :", player2.name, player1.name);
        println!("1: -5 de speed");
        println!("2: -5 de strength");
        print!("Votre choix (1 ou 2): ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                if player1.speed >= 5 {
                    player1.speed -= 5;
                } else {
                    player1.speed = 0;
                }
                println!("{} subit -5 de speed.", player1.name);
            },
            "2" => {
                if player1.strength >= 5 {
                    player1.strength -= 5;
                } else {
                    player1.strength = 0;
                }
                println!("{} subit -5 de strength.", player1.name);
            },
            _ => println!("Choix invalide. Aucun poison appliqué."),
        }
    }

    // Affichage final des statistiques après la manche
    println!("\n--- Fin de la Manche ---");
    println!("{}: Vitality: {}, Speed: {}, Strength: {}", player1.name, player1.vitality, player1.speed, player1.strength);
    println!("{}: Vitality: {}, Speed: {}, Strength: {}", player2.name, player2.vitality, player2.speed, player2.strength);
}
