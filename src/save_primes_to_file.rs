use std::fs::File;
use std::io::{self, Write};

pub fn save_primes_to_file(primes: &Vec<u128>, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;

    let primes_string: String = primes.iter()
        .map(|prime| prime.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    file.write_all(primes_string.as_bytes())?;

    Ok(())
}