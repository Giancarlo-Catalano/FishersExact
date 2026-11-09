use std::fmt;
use std::fmt::Display;
use crate::{contingency_matrix, get_primes_until};

pub(crate) struct VIN {
    // VIN = Vec of i32, interpret as power of Naturals
    // the first item is the power of 2.
    pub(crate) powers: Vec<i32>,
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

    pub(crate) fn from_contingency_matrix(matrix: &contingency_matrix::ContingencyMatrix) -> VIN {
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

pub(crate) struct VIP {
    // vector of integers, these being powers of primes
    // the first prime is 2
    pub(crate) powers: Vec<i32>,
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
    pub(crate) fn from_VIN(vin: &VIN) -> VIP {
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

    pub(crate) fn as_f64(&self) -> f64 {
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
