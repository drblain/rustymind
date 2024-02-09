use rand;

pub fn generate_random_number(difficulty : u8) {
    let base : u32 = 10;
    let upper_bound : u32 = base.pow(difficulty) + 1;
    println!("Difficulty is {difficulty}");
}