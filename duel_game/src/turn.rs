use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;
/// Structure contenant le résultat d’un arrêt de compteur pour un objectif.
pub struct TurnResult {
    pub counter: u8,   // Valeur du compteur au moment de l’arrêt.
    pub miss: u32,  // Nombre de fois où le compteur a dépassé 100 et a été réinitialisé (miss).
}


/// Fonction qui simule la mécanique d’un objectif pour un joueur.
/// Le délai entre chaque incrément est déterminé par `player_speed` (en ms).
///
/// La fonction attend d’abord que le joueur appuie sur ENTER pour démarrer le compteur,
/// lance un thread qui incrémente le compteur, puis attend un second appui sur ENTER pour l’arrêter.
/// Elle retourne la valeur finale du compteur ainsi que le nombre de "miss".

pub fn play_objective(player_speed: u32) -> io::Result<TurnResult> {
    println!("Appuyez sur ENTER pour démarrer cet objectif...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // propagation de l'erreur avec ?

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

    println!("\nAppuyez sur ENTER pour arrêter le compteur...");
    input.clear();
    io::stdin().read_line(&mut input)?; // nouvelle lecture, propagation de l'erreur

    stop_flag.store(true, Ordering::Relaxed);
    handle.join().unwrap();

    let final_counter = *counter.lock().unwrap();
    let final_miss = miss.load(Ordering::Relaxed);
    println!("\nObjectif arrêté à : {} (Miss: {})", final_counter, final_miss);

    Ok(TurnResult {
        counter: final_counter,
        miss: final_miss,
    })
}
