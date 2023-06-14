use std::time::Instant;

fn sieve_primes(n: usize) -> usize {
    let mut prime = vec![true; n+1];

    for p in 2..=n {
        if p * p > n {
            break; 
        }

        if prime[p] == true {
            for i in ((p * p)..=n).step_by(p) {
                prime[i] = false; 
            }
        }
    }

    let mut highest = 2;
    for i in 2..=n {
        if prime[i] == true {
            highest = i;
        }
    }

    highest
}

fn time_sieve(exp: usize) {
    let n = 1000 * exp;
    let start = Instant::now();
    let sieve = sieve_primes(n);
    let elapsed = start.elapsed();

    println!("Sieve took {:?} ({n} max number, {sieve} highest prime)", elapsed);
}

fn main() {
    for i in 1..=1000 {
        time_sieve(i);
    }
}
