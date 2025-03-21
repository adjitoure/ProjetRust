use clap::Parser;
use log::{debug, info};
use std::io::{self, Write};

mod player;
mod objectives;
mod turn;
mod score;
mod game;

use player::Player;

/// Jeu de duel en Rust
#[derive(Parser, Debug)]
#[command(name = "Duel Game", about = "Jeu de duel en Rust", version = "0.1")]
struct Args {
    /// Nom du premier joueur.
    #[arg(long, default_value = "Michel")]
    name1: String,
    /// Nom du deuxième joueur.
    #[arg(long, default_value = "Jacque")]
    name2: String,
    /// Vitalité des joueurs.
    #[arg(long, default_value_t = 50)]
    vitality: u32,
    /// Vitesse des joueurs (délai d’incrémentation en ms).
    #[arg(long, default_value_t = 50)]
    speed: u32,
    /// Force des joueurs.
    #[arg(long, default_value_t = 50)]
    strength: u32,
    /// Nombre d'objectifs par manche.
    #[arg(long, default_value_t = 5)]
    objectifs: u8,
}

fn main() {
    env_logger::init();
    
    let args = Args::parse();
    let mut player1 = Player::new(args.name1, args.vitality, args.speed, args.strength);
    let mut player2 = Player::new(args.name2, args.vitality, args.speed, args.strength);

    println!("Les joueurs ont été initialisés :");
    println!("{}: {:?}", player1.name, player1);
    println!("{}: {:?}", player2.name, player2);

    // Boucle de manches tant que les deux joueurs ont de la vitalité.
    loop {
        game::play_round(&mut player1, &mut player2, args.objectifs);

        // Vérifier si un joueur a épuisé sa vitalité
        if player1.vitality == 0 || player2.vitality == 0 {
            println!("Un joueur n'a plus de vitalité. Fin de la partie !");
            break;
        }
        
        // Demander si on souhaite relancer une manche
        print!("Relancer une manche ? [Y/N] : ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            break;
        }
    }

    // Proposition de relancer une partie
    print!("Relancer une nouvelle partie ? [Y/N] : ");
    io::stdout().flush().unwrap();
    let mut new_game = String::new();
    io::stdin().read_line(&mut new_game).unwrap();
    if new_game.trim().eq_ignore_ascii_case("y") {
        println!("Fonctionnalité à implémenter : redémarrage de la partie.");
        // Ici, vous pourriez réinitialiser les joueurs et relancer la boucle.
    } else {
        println!("Merci d'avoir joué !");
    }
}
