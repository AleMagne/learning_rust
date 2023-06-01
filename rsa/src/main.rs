fn main() {
    let p: i64 = generate_prime_number();
    let q: i64 = generate_prime_number();
    let n: i64 = p*q;
    let phi: i64 = (p-1)*(q-1);
    let public_key: (i64, i64) = (generate_public_key(phi), n);
    let private_key: (i64, i64) = (mcdyx(phi, public_key.0) % phi, n);

    let message_cl: i64 = 573;
    print!("Il messaggio è {message_cl}\n");
    let message_cr: i64 = encrypt(message_cl, public_key);
    print!("Il messaggi criptato è {message_cr}\n");
    let message_decr: i64 = decrypt(message_cr, private_key);
    print!("Alla fine troviamo {message_decr}\n");
}

fn encrypt(message: i64, pu_key :(i64, i64)) -> i64{
    (message^pu_key.0) % pu_key.1
}

fn decrypt(message: i64, pr_key :(i64, i64)) -> i64{
    (message^pr_key.0) % pr_key.1
}

fn generate_public_key(n :i64) -> i64 {
    loop{
        let res: i64 = rand::random::<i64>() % n;
        if mcdyx(res, n) == 1{
            return res;
        }
    }
}

fn mcdyx(a:i64, b:i64) -> i64{
    let mut z: i64 = a;
    let mut w: i64 = b;
    let mut r: i64 = 0;
    while w != 0{
        r = z % w;
        z = w;
        w = r;
    }
    z
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i: i64 = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn generate_prime_number() -> i64 {
    loop {
        let random_number: i64 = (rand::random::<i64>() % 1000000)+10000;
        if is_prime(random_number) {
            return random_number;
        }
    }
}