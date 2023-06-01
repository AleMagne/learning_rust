fn main() {
    let prime_number = generate_prime_number();
    println!("Numero primo generato: {}", prime_number);
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn generate_prime_number() -> u64 {
    loop {
        let random_number: u64 = (rand::random::<u64>() % 1000000)+10000;
        if is_prime(random_number) {
            return random_number;
        }
    }
}

