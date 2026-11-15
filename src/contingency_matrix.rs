use std::cmp::min;
use std::fmt;

#[derive(Clone)]
pub struct ContingencyMatrix {
    pub n_rows: usize,
    pub n_cols: usize,
    pub rows: Vec<Vec<i32>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SumsForContingencyMatrix {
    pub row_sums: Vec<i32>,
    pub col_sums: Vec<i32>,
    pub total_sum: i32,
}



impl ContingencyMatrix {
    pub fn from_elements(rows: Vec<Vec<i32>>) -> ContingencyMatrix {
        if rows.len() == 0 {
            return ContingencyMatrix {n_rows: 0, n_cols: 0, rows: Vec::new()}
        }

        ContingencyMatrix {
            n_rows: rows.len(),
            n_cols: rows[0].len(),
            rows: rows
        }
    }

    #[allow(dead_code)]
    fn get_biggest_value(&self) -> i32 {
        //if there is NOT A SINGLE VALUE, let it panic.
        *self.rows.iter().flatten().max().unwrap()
    }

    pub fn get_sums(&self) -> SumsForContingencyMatrix {
        let mut result = SumsForContingencyMatrix {
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

    fn zeros(n_rows: usize, n_cols: usize) -> ContingencyMatrix {
        ContingencyMatrix{n_rows: n_rows,
            n_cols: n_cols,
            rows:vec![vec![0; n_cols]; n_rows]}
    }


    fn increment_for_enumeration(self: &mut Self, max_values_flat: &Vec<i32>) -> bool {
        // returns false if there is no more.
        let mut current_flat_index = 0;
        let max_index = self.n_rows * self.n_cols;
        while current_flat_index < max_index {
            // increment the current value
            let row_index = current_flat_index / self.n_cols;
            let col_index = current_flat_index % self.n_cols;
            let current_value: &mut i32 = &mut self.rows[row_index][col_index];
            let max_for_cell = max_values_flat[current_flat_index];
            if *current_value == max_for_cell {
                *current_value = 0;
                current_flat_index += 1;
            }
            else {
                *current_value += 1;
                return true;
            }
        }
        // if we escape the loop, it means we reached the end
        return false;
    }


}

impl fmt::Display for ContingencyMatrix {

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

impl SumsForContingencyMatrix {

    fn get_n_rows_cols(&self) -> (usize, usize) {
        let r_count = self.row_sums.len();
        let c_count = self.col_sums.len();
        (r_count, c_count)
    }

    fn get_trivial_max_for_each_cell(&self) -> Vec<i32> {
        // returns the flat vector

        let (n_rows, n_cols) = self.get_n_rows_cols();

        let mut max_values_result: Vec<i32> = Vec::new();
        for row_index in 0..n_rows {
            for col_index in 0..n_cols {
                max_values_result.push(
                    min(self.row_sums[row_index],
                        self.col_sums[col_index]));
            }
        }
        max_values_result
    }


    pub fn get_all_matching_contingency_tables(&self) -> Vec<ContingencyMatrix> {

        let mut result = Vec::new();


        let max_values_flat : Vec<i32>= self.get_trivial_max_for_each_cell();
        let (n_rows, n_cols) = self.get_n_rows_cols();
        {
            let mut current_table = ContingencyMatrix::zeros(n_rows, n_cols);
            let mut conditionally_register_if_satisfactory = |table: &mut ContingencyMatrix,result:&mut Vec<ContingencyMatrix>| {
                if table.get_sums() == *self {
                    result.push(table.clone());
                }
            };

            loop {
                conditionally_register_if_satisfactory(&mut current_table, &mut result);
                let should_continue = (&mut current_table).increment_for_enumeration(&max_values_flat);
                if should_continue {
                    break;
                }
            }
        }
        result
    }

    pub fn count_all_matching_contingency_tables(&self) -> i64 {

        let mut result = 0;
        let (n_rows, n_cols) = self.get_n_rows_cols();

        let max_values_flat : Vec<i32>= self.get_trivial_max_for_each_cell();
        let (n_rows, n_cols) = self.get_n_rows_cols();
        {
            let mut current_table = ContingencyMatrix::zeros(n_rows, n_cols);
            let mut conditionally_register_if_satisfactory = |table: &mut ContingencyMatrix| {
                if table.get_sums() == *self {
                    result+=1;
                }
            };

            loop {
                conditionally_register_if_satisfactory(&mut current_table);
                if !current_table.increment_for_enumeration(&max_values_flat) {
                    break;
                }
            }

        }
        result
    }
}



// fn contingency_matrix_main() {
//     let sums = SumsForContingencyMatrix{
//         row_sums: vec![2, 2, 2],
//         col_sums: vec![3, 3, 3],
//         total_sum: 6
//     };
// }
