// This will be a simple script to calculate Fisher's Exact test.

use std::fmt;

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

fn add_elementwise_to_int_vector(vec_a: &mut Vec<i32>, vec_b: &Vec<i32>) -> () {
    // vec_a += vec_b
    for i in 0..vec_a.len() {
        vec_a[i] += vec_b[i];
    }
}

fn incr_first_n_of_int_vector(vec_a: &mut Vec<i32>, n: i32) -> () {
    // increments the first n items of the vector
    for i in 0..n {
        vec_a[i as usize] += 1;
    }
}

fn decr_first_n_of_int_vector(vec_a: &mut Vec<i32>, n:i32) -> () {
    // decrements the first n
    for i in 0..n {
        vec_a[i as usize] -= 1;
    }
}




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

fn main() {
    for n in 0..20 {
        let primes_until_n = get_primes_until(n);
        println!("Primes until {} are {:?}", n, primes_until_n);
    }

    let matrix = CoOccurrenceMatrix::from_elements(
        vec![vec![1, 2],
                  vec![3, 6]]
    );

    println!("{}", matrix);
    println!("{:?}", matrix.get_sums());


}
