#[derive(Debug, Clone)]
pub struct Value {
    value: u8,
    first_col: usize,
    second_col: usize,
    col_sum: usize,
}

#[derive(Debug, Clone)]
pub struct Calculation {
    first_row: Vec<u8>,
    second_row: Vec<u8>,
}

pub impl Calculation {
    fn multiply(&self) -> Vec<u8> {
        let mut result: Vec<Value> = Vec::new();
        let mut max_colsum_test: usize = 0;
        for (index_first_row, entry_first_row) in self.first_row.iter().enumerate() {
            for (index_second_row, entry_second_row) in self.second_row.iter().enumerate() {
                let multiplication = entry_first_row * entry_second_row;
                let col_sum = index_first_row + index_second_row;
                let value = Value {
                    value: multiplication,
                    first_col: index_first_row,
                    second_col: index_second_row,
                    col_sum,
                };
                if col_sum > max_colsum_test {
                    max_colsum_test = col_sum;
                }
                result.insert(0, value);
            }
        }
        let normalised_vector = normalise_vector(result);
        dbg!(&normalised_vector);
        return vec![0];
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_multiply_1() {
        let calculation = Calculation {
            first_row: vec![1, 2, 1],
            second_row: vec![1, 0],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 2, 1, 0])
    }

    #[test]
    fn test_multiply_2() {
        let calculation = Calculation {
            first_row: vec![1, 2],
            second_row: vec![1, 0],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 2, 0])
    }

    #[test]
    fn test_multiply_3() {
        let calculation = Calculation {
            first_row: vec![1],
            second_row: vec![1, 0],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 0])
    }

    #[test]
    fn test_multiply_4() {
        let calculation = Calculation {
            first_row: vec![1, 2],
            second_row: vec![1, 2],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 4, 4])
    }

    #[test]
    fn test_multiply_5() {
        let calculation = Calculation {
            first_row: vec![1, 2, 5],
            second_row: vec![2],
        };
        debug_assert_eq!(calculation.multiply(), vec![2, 5, 0])
    }

    #[test]
    fn test_multiply_6() {
        let calculation = Calculation {
            first_row: vec![1, 2, 1],
            second_row: vec![9],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 0, 8, 9])
    }
}
