use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::BTreeMap;

pub fn generate_bonus_objectives(count: usize) -> BTreeMap<char, u8> {
    // Ensemble de touches autorisées
    let keys: Vec<char> = "azertyuiopqsdfghjklmwxcvbn".chars().collect();
    let mut rng = rand::thread_rng();
    let mut available_keys = keys.clone();
    available_keys.shuffle(&mut rng);
    let mut map = BTreeMap::new();
    for key in available_keys.into_iter().take(count) {
        map.insert(key, rng.gen_range(0..=100));
    }
    map
}
