use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

pub struct BonusTurnResult {
    pub counter: u8,
    pub miss: u32,
    pub correct: bool,
}

pub fn play_bonus_objective(expected: char, player_speed: u32) -> io::Result<BonusTurnResult> {
    println!("Appuyez sur ENTER pour démarrer l'objectif pour la touche '{}'", expected);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let counter = Arc::new(Mutex::new(0u8));
    let miss = Arc::new(AtomicU32::new(0));
    let stop_flag = Arc::new(AtomicBool::new(false));

    let counter_thread = Arc::clone(&counter);
    let miss_thread = Arc::clone(&miss);
    let stop_flag_thread = Arc::clone(&stop_flag);

    let handle = thread::spawn(move || {
        while !stop_flag_thread.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(player_speed as u64));
            let mut count = counter_thread.lock().unwrap();
            *count += 1;
            if *count > 100 {
                *count = 0;
                miss_thread.fetch_add(1, Ordering::Relaxed);
            }
            print!("\rCompteur: {} | Miss: {}", *count, miss_thread.load(Ordering::Relaxed));
            io::stdout().flush().unwrap();
        }
    });

    println!("\nAppuyez sur la touche correspondant à '{}' pour arrêter le compteur :", expected);
    input.clear();
    io::stdin().read_line(&mut input)?;
    stop_flag.store(true, Ordering::Relaxed);
    handle.join().unwrap();

    let final_counter = *counter.lock().unwrap();
    let final_miss = miss.load(Ordering::Relaxed);
    // On récupère le premier caractère de l'input (après l'arrêt)
    let pressed = input.trim().chars().next().unwrap_or(' ');
    let correct = pressed == expected;

    if !correct {
        println!("\nMauvaise touche ! Attendu '{}', appuyé '{}'. Score pour cet objectif sera 0.", expected, pressed);
    } else {
        println!("\nBonne touche !");
    }
    println!("Objectif terminé: Compteur: {} (Miss: {})", final_counter, final_miss);

    Ok(BonusTurnResult { counter: final_counter, miss: final_miss, correct })
}
