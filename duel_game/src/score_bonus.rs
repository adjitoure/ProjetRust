use crate::bonus_turn::BonusTurnResult;

pub fn calculate_bonus_score(target: u8, result: &BonusTurnResult, force: u32) -> u32 {
    if !result.correct {
        0
    } else {
        let counter = result.counter;
        let miss = result.miss;
        let diff_raw = if target >= counter { target - counter } else { counter - target };
        let diff = if diff_raw > 50 { 100 - diff_raw } else { diff_raw };

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

        (base as u32 + force) / (miss + 1)
    }
}
