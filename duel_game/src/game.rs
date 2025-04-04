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

pub fn play_round(player1: &mut Player, player2: &mut Player, nb_objectifs: u8) -> io::Result<()> {
    println!("----- Nouvelle Manche -----");

    // Tour du joueur 1
    println!("\nAu tour de {} (Vitality: {}, Speed: {}, Strength: {})", player1.name, player1.vitality, player1.speed, player1.strength);
    let objectives1 = generate_objectives(nb_objectifs);
    println!("Objectifs: {:?}", objectives1);

    // Utilisation d'un itérateur et d'une closure pour collecter les scores
    let scores1: io::Result<Vec<u32>> = objectives1.iter().enumerate().map(|(i, target)| {
        println!("\n--- {} - Objectif {}: Cible {} ---", player1.name, i + 1, target);
        let result = play_objective(player1.speed)?;
        let score_value = calculate_score(*target, &result, player1.strength);
        println!("Score pour cet objectif: {}", score_value);
        Ok(score_value)
    }).collect();

    let scores1 = scores1?;
    let average_score1 = if scores1.is_empty() {
        0
    } else {
        (scores1.iter().sum::<u32>() as f32 / scores1.len() as f32).ceil() as u32
    };
    println!("Score moyen de {}: {}", player1.name, average_score1);

    // Tour du joueur 2
    println!("\nAu tour de {} (Vitality: {}, Speed: {}, Strength: {})", player2.name, player2.vitality, player2.speed, player2.strength);
    let objectives2 = generate_objectives(nb_objectifs);
    println!("Objectifs: {:?}", objectives2);

    let scores2: io::Result<Vec<u32>> = objectives2.iter().enumerate().map(|(i, target)| {
        println!("\n--- {} - Objectif {}: Cible {} ---", player2.name, i + 1, target);
        let result = play_objective(player2.speed)?;
        let score_value = calculate_score(*target, &result, player2.strength);
        println!("Score pour cet objectif: {}", score_value);
        Ok(score_value)
    }).collect();

    let scores2 = scores2?;
    let average_score2 = if scores2.is_empty() {
        0
    } else {
        (scores2.iter().sum::<u32>() as f32 / scores2.len() as f32).ceil() as u32
    };
    println!("Score moyen de {}: {}", player2.name, average_score2);

    // Comparaison et application des conséquences
    if average_score1 == average_score2 {
        println!("La manche est nulle, aucun changement de vitalité.");
    } else if average_score1 > average_score2 {
        let diff = average_score1 - average_score2;
        println!("{} gagne la manche !", player1.name);
        println!("{} perd {} points de vitalité.", player2.name, diff);
        player2.vitality = player2.vitality.saturating_sub(diff);
        println!("{} choisissez un poison à appliquer à {} :", player1.name, player2.name);
        println!("1: -5 de speed");
        println!("2: -5 de strength");
        print!("Votre choix (1 ou 2): ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        match choice.trim() {
            "1" => {
                player2.speed = player2.speed.saturating_sub(5);
                println!("{} subit -5 de speed.", player2.name);
            },
            "2" => {
                player2.strength = player2.strength.saturating_sub(5);
                println!("{} subit -5 de strength.", player2.name);
            },
            _ => println!("Choix invalide. Aucun poison appliqué."),
        }
    } else {
        let diff = average_score2 - average_score1;
        println!("{} gagne la manche !", player2.name);
        println!("{} perd {} points de vitalité.", player1.name, diff);
        player1.vitality = player1.vitality.saturating_sub(diff);
        println!("{} choisissez un poison à appliquer à {} :", player2.name, player1.name);
        println!("1: -5 de speed");
        println!("2: -5 de strength");
        print!("Votre choix (1 ou 2): ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        match choice.trim() {
            "1" => {
                player1.speed = player1.speed.saturating_sub(5);
                println!("{} subit -5 de speed.", player1.name);
            },
            "2" => {
                player1.strength = player1.strength.saturating_sub(5);
                println!("{} subit -5 de strength.", player1.name);
            },
            _ => println!("Choix invalide. Aucun poison appliqué."),
        }
    }
    
    println!("\n--- Fin de la Manche ---");
    println!("{}: Vitality: {}, Speed: {}, Strength: {}", player1.name, player1.vitality, player1.speed, player1.strength);
    println!("{}: Vitality: {}, Speed: {}, Strength: {}", player2.name, player2.vitality, player2.speed, player2.strength);
    
    Ok(())
}

pub fn play_round_multi(players: &mut Vec<Player>, nb_objectifs: u8) -> io::Result<()> {
    println!("----- Nouvelle Manche (Multi-joueurs) -----");
    let mut scores = Vec::new();
    for player in players.iter_mut() {
        println!("\nAu tour de {} (Vitality: {}, Speed: {}, Strength: {})", player.name, player.vitality, player.speed, player.strength);
        let objectives = generate_objectives(nb_objectifs);
        println!("Objectifs: {:?}", objectives);
        let player_scores: io::Result<Vec<u32>> = objectives.iter().enumerate().map(|(i, target)| {
            println!("\n--- {} - Objectif {}: Cible {} ---", player.name, i + 1, target);
            let result = play_objective(player.speed)?;
            let score = calculate_score(*target, &result, player.strength);
            println!("Score pour cet objectif: {}", score);
            Ok(score)
        }).collect();
        let player_scores = player_scores?;
        let avg = if player_scores.is_empty() {
            0
        } else {
            (player_scores.iter().sum::<u32>() as f32 / player_scores.len() as f32).ceil() as u32
        };
        println!("Score moyen de {}: {}", player.name, avg);
        scores.push(avg);
    }
    // Déterminer le score maximum
    let max_score = scores.iter().cloned().max().unwrap_or(0);
    // Pour chaque joueur non gagnant, appliquer une perte de vitalité égale à la différence.
    for (i, player) in players.iter_mut().enumerate() {
        if scores[i] < max_score {
            let diff = max_score - scores[i];
            println!("{} perd {} points de vitalité.", player.name, diff);
            player.vitality = player.vitality.saturating_sub(diff);
        }
    }
    // Le gagnant choisit un joueur à pénaliser.
    if let Some((winner_idx, _winner)) = scores.iter().enumerate().find(|&(_, &score)| score == max_score) {
        println!("{} est le gagnant de la manche !", players[winner_idx].name);
        println!("{} choisissez un joueur à pénaliser (saisissez le numéro correspondant) :", players[winner_idx].name);
        for (i, player) in players.iter().enumerate() {
            if i != winner_idx {
                println!("{}: {}", i, player.name);
            }
        }
        print!("Votre choix: ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        if let Ok(idx) = choice.trim().parse::<usize>() {
            if idx < players.len() && idx != winner_idx {
                println!("{} choisissez un poison pour {} : 1: -5 de speed, 2: -5 de strength", players[winner_idx].name, players[idx].name);
                print!("Votre choix (1 ou 2): ");
                io::stdout().flush()?;
                let mut poison = String::new();
                io::stdin().read_line(&mut poison)?;
                match poison.trim() {
                    "1" => {
                        players[idx].speed = players[idx].speed.saturating_sub(5);
                        println!("{} subit -5 de speed.", players[idx].name);
                    },
                    "2" => {
                        players[idx].strength = players[idx].strength.saturating_sub(5);
                        println!("{} subit -5 de strength.", players[idx].name);
                    },
                    _ => println!("Choix invalide. Aucun poison appliqué."),
                }
            }
        }
    }

    println!("\n--- Fin de la Manche ---");
    for player in players.iter() {
        println!("{}: Vitality: {}, Speed: {}, Strength: {}", player.name, player.vitality, player.speed, player.strength);
    }
    Ok(())
}
