// This will be a simple script to calculate Fisher's Exact test.
// See this page for more information:
//      https://peterstatistics.com/Terms/Tests/FisherExactTest.html
// In short: Fisher's test is for 2x2, and Fisher-Freeman-Halton is for the nxm case.


// This is tricky to calculate because it requires huge factorials,
// that very quickly break IEEE754 ranges.
// The annoying part is that the end result is a division between HUGE numbers,
// and it should be in range [0,1].
// Yes, we could use logs and stuff like that, but we lose a lot of precision...
// so we use symbolic exact computation.

mod contingency_matrix;
mod product_of_factors;


fn get_primes_until(n: i32) -> Vec<i32> {
    // Implements a Sieve of Eratostenes
    if n < 1 {
        return Vec::new();
    }


    let mut is_prime_boolean_array = vec![true; (n+1) as usize];
    // represents whether every item 0..=n is prime or not
    is_prime_boolean_array[0] = false;
    is_prime_boolean_array[1] = false;


    let mut primes: Vec<i32> = Vec::new();

    for i in 0..=n {
        let is_prime = is_prime_boolean_array[i as usize];
        if !is_prime {
            continue;
        }
        primes.push(i); // must be a prime
        // we then mark every multiple of i as a non-integer
        // we start from i * i, until i * (n/i). Call that multiplier k
        for k in i..=(n/i) {
            is_prime_boolean_array[(i*k) as usize] = false;
        }
    }
    primes
}

// fn add_elementwise_to_int_vector(vec_a: &mut Vec<i32>, vec_b: &Vec<i32>) -> () {
//     // vec_a += vec_b
//     for i in 0..vec_a.len() {
//         vec_a[i] += vec_b[i];
//     }
// }










fn main() {


    let matrix = contingency_matrix::ContingencyMatrix::from_elements(
        vec![vec![0, 10],
                  vec![12, 2]]
    );

    println!("matrix = \n4{}", matrix);
    println!("sums = {:?}", matrix.get_sums());

    let vin = product_of_factors::VIN::from_contingency_matrix(&matrix);
    println!("vin.powers = {:?}", vin.powers);
    println!("vin = {}", vin);

    let vip = product_of_factors::VIP::from_VIN(&vin);
    println!("vip.powers = {:?}", vip.powers);
    println!("vip = {}", vip);

    println!("as float = {}", vip.as_f64());



}