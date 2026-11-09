use std::fmt;

pub(crate) struct ContingencyMatrix {
    pub(crate) n_rows: usize,
    pub(crate) n_cols: usize,
    pub(crate) rows: Vec<Vec<i32>>,
}

#[derive(Debug)]
pub(crate) struct SumsForContingencyMatrix {
    pub(crate) row_sums: Vec<i32>,
    pub(crate) col_sums: Vec<i32>,
    pub(crate) total_sum: i32,
}



impl ContingencyMatrix {
    pub(crate) fn from_elements(rows: Vec<Vec<i32>>) -> ContingencyMatrix {
        if rows.len() == 0 {
            return ContingencyMatrix {n_rows: 0, n_cols: 0, rows: Vec::new()}
        }

        ContingencyMatrix {
            n_rows: rows.len(),
            n_cols: rows[0].len(),
            rows: rows
        }
    }

    fn get_biggest_value(&self) -> i32 {
        //if there is NOT A SINGLE VALUE, let it panic.
        *self.rows.iter().flatten().max().unwrap()
    }

    pub(crate) fn get_sums(&self) -> SumsForContingencyMatrix {
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

