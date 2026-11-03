// This will be a simple script to calculate Fisher's Exact test.


fn get_primes_until(n: i32) -> Vec<i32> {
    // Implements a Sieve of Eratostenes
    if n < 1 {
        return Vec::new();
    }


    let mut is_prime_boolean_array = vec![true; (n+1) as usize];
    // represents whether every item 0..=n is prime or not
    is_prime_boolean_array[0] = false;
    is_prime_boolean_array[1] = false;


    let mut primes: Vec::<i32> = Vec::new();

    for i in 0..=n {
        let is_prime = is_prime_boolean_array[i as usize];
        if !is_prime {
            continue;
        }
        primes.push(i as i32); // must be a prime
        // we then mark every multiple of i as a non-integer
        // we start from i * i, until i * (n/i). Call that multiplier k
        for k in i..=(n/i) {
            is_prime_boolean_array[(i*k) as usize] = false;
        }
    }
    primes
}


fn main() {
    for n in 0..20 {
        let primes_until_n = get_primes_until(n);
        println!("Primes until {} are {:?}", n, primes_until_n);
    }

}
