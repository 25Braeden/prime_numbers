use std::u128;
use crate::find_prime::find_prime;

pub fn make_prime_list(list: &mut Vec<u128>, starting_from: u128, max_search: u128) {
    if starting_from == 0 || starting_from == 1 {
        list.push(2);
    }
    let mut last_prime = if starting_from < 2 { 2 } else { starting_from };
    while let Some(prime) = find_prime(last_prime, max_search) {
        list.push(prime);
        last_prime = prime;
    }
}
