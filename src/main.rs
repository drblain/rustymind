mod scanner;
mod mastermind;

fn main() {
    let diff : u8 = 4;
    mastermind::generate_random_number(diff);
}
