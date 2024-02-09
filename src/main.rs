use std::env;

mod scanner;
mod mastermind;

fn main() {
    let args : Vec<String> = env::args().collect();
    let num_args = args.len();

    dbg!(num_args);

    let diff : u32 = 0;
    let random_value : u32 = mastermind::generate_random_number(diff);

    println!("Generated random number is {random_value}");
}
