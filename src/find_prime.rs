use std::u128;
pub fn find_prime(last_prime: u128, search_cap: u128) -> Option<u128> {
    for num in last_prime + 1..=search_cap {
        let mut is_prime = true;
        let limit = (num as f64).sqrt() as u128;
        for j in 2..=limit {
            if num % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            return Some(num)
        }
    }
    None
}
