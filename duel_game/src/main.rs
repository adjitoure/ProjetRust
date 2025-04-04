use clap::Parser;
use log::{debug, info};
use std::error::Error;
use std::io::{self, Write};

mod player;
mod objectives;
mod turn;
mod score;
mod game;
mod bonus_objectives;
mod bonus_turn;
mod score_bonus;

use player::Player;

#[derive(Parser, Debug)]
#[command(name = "Duel Game", about = "Jeu de duel en Rust", version = "0.1")]
struct Args {
    #[arg(long, default_value = "Michel")]
    name1: String,
    #[arg(long, default_value = "Jacque")]
    name2: String,
    #[arg(long, default_value_t = 50)]
    vitality: u32,
    #[arg(long, default_value_t = 50)]
    speed: u32,
    #[arg(long, default_value_t = 50)]
    strength: u32,
    #[arg(long, default_value_t = 5)]
    objectifs: u8,
    /// Active le mode multi-joueurs. Dans ce mode, l'argument --players (noms séparés par des virgules) est utilisé.
    #[arg(long)]
    multi: bool,
    /// Liste de noms de joueurs séparés par des virgules (ex: "Alice,Bob,Charlie")
    #[arg(long)]
    players: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = Args::parse();

    if args.multi {
        // Mode multi-joueurs
        let names = args.players.unwrap_or_else(|| format!("{},{}", args.name1, args.name2));
        let player_names: Vec<&str> = names.split(',').map(|s| s.trim()).collect();
        let mut players: Vec<Player> = player_names
            .iter()
            .map(|&name| Player::new(name.to_string(), args.vitality, args.speed, args.strength))
            .collect();

        println!("Mode multi-joueurs activé :");
        for player in &players {
            println!("{}: {:?}", player.name, player);
        }

        loop {
            game::play_round_multi(&mut players, args.objectifs)?;
            if players.iter().any(|p| p.vitality == 0) {
                println!("Au moins un joueur n'a plus de vitalité. Fin de la partie !");
                break;
            }
            print!("Relancer une manche ? [Y/N] : ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                break;
            }
        }
    } else {
        // Mode classique 2 joueurs
        let mut player1 = Player::new(args.name1, args.vitality, args.speed, args.strength);
        let mut player2 = Player::new(args.name2, args.vitality, args.speed, args.strength);
        println!("Les joueurs ont été initialisés :");
        println!("{}: {:?}", player1.name, player1);
        println!("{}: {:?}", player2.name, player2);

        loop {
            game::play_round(&mut player1, &mut player2, args.objectifs)?;
            if player1.vitality == 0 || player2.vitality == 0 {
                println!("Un joueur n'a plus de vitalité. Fin de la partie !");
                break;
            }
            print!("Relancer une manche ? [Y/N] : ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                break;
            }
        }
    }

    print!("Relancer une nouvelle partie ? [Y/N] : ");
    io::stdout().flush()?;
    let mut new_game = String::new();
    io::stdin().read_line(&mut new_game)?;
    if new_game.trim().eq_ignore_ascii_case("y") {
        println!("Fonctionnalité à implémenter : redémarrage de la partie.");
    } else {
        println!("Merci d'avoir joué !");
    }
    Ok(())
}
