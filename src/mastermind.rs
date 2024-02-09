use rand::Rng;

const NUMERIC_BASE : u32 = 10;

const MIN_DIFFICULTY: u32 = 0;
const MAX_DIFFICULTY: u32 = 4;

fn is_difficulty_valid(difficulty : u32) -> bool {
    return difficulty >= MIN_DIFFICULTY && difficulty <= MAX_DIFFICULTY;
}

// Temporarily public
pub fn generate_random_number(difficulty : u32) -> u32 {
    let mut validated_difficulty : u32 = 0;

    if is_difficulty_valid(difficulty) {
        validated_difficulty = difficulty;
    }

    let upper_bound : u32 = NUMERIC_BASE.pow(validated_difficulty + 4) + 1;
    return rand::thread_rng().gen_range(0..upper_bound);
}

pub fn run_game() {
    
}