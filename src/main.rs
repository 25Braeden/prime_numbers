use std::io::{self, Write};
use std::io::stdin;
use std::time::Instant;

use make_prime_list::make_prime_list;
use crate::save_primes_to_file::save_primes_to_file;
mod save_primes_to_file;
mod find_prime;
mod make_prime_list;
fn main() {
    print!("Enter a non-negative starting point: ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Failed to read line.");

    let starting_point: u128 = match user_input.trim().parse::<u128>() {
        Ok(number) => number,
        Err(_) => {
            println!("Something went wrong.");
            return;
        }
    };

    print!("Enter a maximum test number: ");
    io::stdout().flush().unwrap();
    let mut user_second_input = String::new();
    stdin().read_line(&mut user_second_input).expect("Failed to read line.");

    let maximum_search: u128 = match user_second_input.trim().parse::<u128>() {
        Ok(number) => number,
        Err(_) => {
            println!("Something went wrong.");
            return;
        }
    };

    let mut primes: Vec<u128> = Vec::new();
    let start = Instant::now();
    make_prime_list(&mut primes, starting_point, maximum_search);
    match save_primes_to_file(&primes, "primes.txt") {
        Ok(_) => println!("Primes were successfully written to primes.txt"),
        Err(e) => println!("Failed to write primes to file: {}", e),
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
