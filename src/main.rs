// This will be a simple script to calculate Fisher's Exact test.

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





struct CoOccurrenceMatrix {
    n_rows: usize,
    n_cols: usize,
    rows: Vec<Vec<i32>>,
}

#[derive(Debug)]
struct SumsForCoOccurrenceMatrix {
    row_sums: Vec<i32>,
    col_sums: Vec<i32>,
    total_sum: i32,
}

impl CoOccurrenceMatrix {
    fn from_elements(rows: Vec<Vec<i32>>) -> CoOccurrenceMatrix {
        if rows.len() == 0 {
            return CoOccurrenceMatrix{n_rows: 0, n_cols: 0, rows: Vec::new()}
        }

        CoOccurrenceMatrix{
            n_rows: rows.len(),
            n_cols: rows[0].len(),
            rows: rows
        }
    }

    fn get_biggest_value(&self) -> i32 {
        //if there is NOT A SINGLE VALUE, let it panic.
        *self.rows.iter().flatten().max().unwrap()
    }

    fn get_sums(&self) -> SumsForCoOccurrenceMatrix {
        let mut result = SumsForCoOccurrenceMatrix {
            row_sums: vec![0; self.n_rows],
            col_sums: vec![0; self.n_cols],
            total_sum: 0,
        };

        for (row_index, row) in self.rows.iter().enumerate() {
            let mut row_sum = 0;
            for (col_index, item) in row.iter().enumerate() {
                row_sum += item;
                result.col_sums[col_index]+=item;
            }
            result.row_sums[row_index] = row_sum;
            result.total_sum += row_sum;
        }
        result
    }
}

impl fmt::Display for CoOccurrenceMatrix {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows.iter() {
            for item in row.iter() {
                write!(f, "{}\t", item)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}


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

    fn from_CoOccurrenceMatrix(matrix: &CoOccurrenceMatrix) -> VIN {
        // I should say somewhere that this is where Fisher's exact formula is
        let sums = matrix.get_sums();
        let mut vin = VIN::zeros((sums.total_sum-1) as usize);
        // product of row sums! divided by ((product of col sums!) * total_sum!)

        // multiply by row sums
        for &row_sum in sums.row_sums.iter() {
            vin.multiply_by_n_factorial(row_sum);
        }

        for &col_sum in sums.col_sums.iter() {
            vin.divide_by_n_factorial(col_sum);
        }
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
}

impl Display for VIP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // we calculate the primes here, but that is very bad.
        let primes = get_primes_until(100);
        // just for debug!

        let mut numerator = 1;
        let mut denominator = 1;
        for (index, &power) in self.powers.iter().enumerate() {
            if power != 0 {
                let prime = primes[index];
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
        let mut vip = VIP{powers: vec![0; primes.len()]};
        for (vin_index, &vin_power) in vin.powers.iter().enumerate() {
            let natural: i32 = (vin_index as i32) + 2;
            for (prime_index, &prime) in primes.iter().enumerate() {
                if natural % prime == 0 {
                    // first we need to count how many times it fits
                    let mut prime_power_multiplier = {
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
}


fn main() {


    let matrix = CoOccurrenceMatrix::from_elements(
        vec![vec![1, 2],
                  vec![3, 4]]
    );

    let matrix_sums = matrix.get_sums();
    let primes = get_primes_until(matrix_sums.total_sum);


    println!("matrix = {}", matrix);
    println!("sums = {:?}", matrix.get_sums());

    let vin = VIN::from_CoOccurrenceMatrix(&matrix);
    println!("vin.powers = {:?}", vin.powers);
    println!("vin = {}", vin);

    let vip = VIP::from_VIN(&vin);
    println!("vip.powers = {:?}", vip.powers);
    println!("vip = {}", vip);

    //   2   3   5
    // [-0, -1, -0]


}