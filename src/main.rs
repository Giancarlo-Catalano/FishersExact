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

use std::cmp::max;
use std::fmt;
use std::fmt::Display;
use std::path::Component::ParentDir;

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

// fn add_elementwise_to_int_vector(vec_a: &mut Vec<i32>, vec_b: &Vec<i32>) -> () {
//     // vec_a += vec_b
//     for i in 0..vec_a.len() {
//         vec_a[i] += vec_b[i];
//     }
// }






struct VIN {
    // VIN = Vec of i32, interpret as power of Naturals
    // the first item is the power of 2.
    powers: Vec<i32>,
}

impl VIN {

    fn zeros(capacity: usize) -> VIN {
        VIN{powers: vec![0; capacity]}
    }

    fn max_base_inclusive(&self) -> i32 {
        return (self.powers.len() as i32 ) + 1;
    }

    fn multiply_by_n_factorial(&mut self, n: i32) -> () {
        if n < 1 {
            panic!("Cannot call multiply_by_n_factorial({})", n);
        }
        // increments the first n items of the vector
        for i in 0..(n-1) {
            self.powers[i as usize] += 1;
        }
    }

    fn divide_by_n_factorial(&mut self, n:i32) -> () {
        if n < 1 {
            panic!("Cannot call divide_by_n_factorial({})", n);
        }
        // decrements the first n
        for i in 0..(n-1) {
            self.powers[i as usize] -= 1;
        }
    }

    fn from_contingency_matrix(matrix: &contingency_matrix::ContingencyMatrix) -> VIN {
        // I should say somewhere that this is where Fisher's exact formula is
        let sums = matrix.get_sums();
        let mut vin = VIN::zeros((sums.total_sum-1) as usize);
        // product of row sums! divided by ((product of col sums!) * total_sum!)

        // multiply by row sums
        for &row_sum in sums.row_sums.iter() {
            vin.multiply_by_n_factorial(row_sum);
        }

        // multiply by col sums
        for &col_sum in sums.col_sums.iter() {
            vin.multiply_by_n_factorial(col_sum);
        }

        // divide by table items
        for &item in matrix.rows.iter().flatten() {
            vin.divide_by_n_factorial(item);
        }

        // divide by total
        vin.divide_by_n_factorial(sums.total_sum);
        vin
    }
}

impl fmt::Display for VIN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut numerator = 1;
        let mut denominator = 1;
        for (index, &power) in self.powers.iter().enumerate() {
            if power != 0 {
                let natural = index + 2;
                if power > 0 {
                    for _ in 0..power {
                        numerator *= natural;
                    }
                }
                else {
                    for _ in 0..(-1*power) {
                        denominator *= natural;
                    }
                }
            }
        }
        writeln!(f, "{}/{}", numerator, denominator)
    }
}

struct VIP {
    // vector of integers, these being powers of primes
    // the first prime is 2
    powers: Vec<i32>,
    primes: Vec<i32>,
}

impl Display for VIP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut numerator = 1;
        let mut denominator = 1;
        for (index, &power) in self.powers.iter().enumerate() {
            if power != 0 {
                let prime = self.primes[index];
                if power > 0 {
                    for _ in 0..power {
                        numerator *= prime;
                    }
                }
                else {
                    for _ in 0..(-1*power) {
                        denominator *= prime;
                    }
                }
            }
        }
        writeln!(f, "{}/{}", numerator, denominator)
    }
}

impl VIP {
    fn from_VIN(vin: &VIN) -> VIP {
        // first, we calculate what primes we need
        let primes = get_primes_until(vin.max_base_inclusive());
        let mut vip = VIP{
            powers: vec![0; primes.len()],
            primes
        };
        for (vin_index, &vin_power) in vin.powers.iter().enumerate() {
            let natural: i32 = (vin_index as i32) + 2;
            for (prime_index, &prime) in vip.primes.iter().enumerate() {
                if natural % prime == 0 {
                    // first we need to count how many times it fits
                    let prime_power_multiplier = {
                        let mut temp_power = 0;
                        let mut temp_natural = natural;
                        while temp_natural % prime == 0 {
                            temp_natural /= prime;
                            temp_power += 1;
                        }
                        temp_power
                    };
                    vip.powers[prime_index] += vin_power * prime_power_multiplier;
                }
            }
        }
        vip
    }

    fn as_f64(&self) -> f64 {
        // we need to be careful to avoid overflows.
        // these two are lists of (prime, power)
        let (mut mult_by, mut div_by) = {
            let mut temp_mult_pairs : Vec<(i32, i32)> = Vec::new();
            let mut temp_div_pairs : Vec<(i32, i32)> = Vec::new();


            for (&prime, &power) in self.primes.iter().zip(self.powers.iter()) {
                if power > 0 {
                    temp_mult_pairs.push((prime, power));
                }
                else if power < 0 {
                    temp_div_pairs.push((prime, -1 * power));
                }
                // otherwise nothing
            }
            (temp_mult_pairs, temp_div_pairs)
        };
        // the value should always be below 1 so when it goes over we divide it
        let mut result : f64 = 1.0;

        //TODO actually implement this properly...but how?

        for (prime, power) in div_by.iter() {
            for _ in 0..*power {
                result /= *prime as f64;
            }
        }
        for (prime, power) in mult_by.iter() {
            for _ in 0..*power {
                result *= *prime as f64;
            }
        }
        result
    }
}





fn main() {


    let matrix = contingency_matrix::ContingencyMatrix::from_elements(
        vec![vec![0, 10],
                  vec![12, 2]]
    );

    let matrix_sums = matrix.get_sums();
    let primes = get_primes_until(matrix_sums.total_sum);


    println!("matrix = \n4{}", matrix);
    println!("sums = {:?}", matrix.get_sums());

    let vin = VIN::from_contingency_matrix(&matrix);
    println!("vin.powers = {:?}", vin.powers);
    println!("vin = {}", vin);

    let vip = VIP::from_VIN(&vin);
    println!("vip.powers = {:?}", vip.powers);
    println!("vip = {}", vip);

    println!("as float = {}", vip.as_f64());



}