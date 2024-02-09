mod scanner;
mod mastermind;

fn main() {
    let diff : u32 = 0;
    let random_value : u32 = mastermind::generate_random_number(diff);

    println!("Generated random number is {random_value}");
}
