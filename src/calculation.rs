use crate::{multiply_vectors::multiply_vectors, normalise_vector::normalise_vector};

#[derive(Debug, Clone)]
pub struct Calculation {
    pub first_row: Vec<u64>,
    pub second_row: Vec<u64>,
}

impl Calculation {
    pub fn multiply(&self) -> Vec<u64> {
        let result = multiply_vectors(self.clone().first_row, self.clone().second_row);
        normalise_vector(result)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_multiply_1() {
        let calculation = Calculation {
            first_row: vec![1, 2, 1],
            second_row: vec![1, 0],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 2, 1, 0])
    }

    #[test]
    fn test_calculate_multiply_2() {
        let calculation = Calculation {
            first_row: vec![1, 2],
            second_row: vec![1, 0],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 2, 0])
    }

    #[test]
    fn test_calculate_multiply_3() {
        let calculation = Calculation {
            first_row: vec![1],
            second_row: vec![1, 0],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 0])
    }

    #[test]
    fn test_calculate_multiply_4() {
        let calculation = Calculation {
            first_row: vec![1, 2],
            second_row: vec![1, 2],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 4, 4])
    }

    #[test]
    fn test_calculate_multiply_5() {
        let calculation = Calculation {
            first_row: vec![1, 2, 5],
            second_row: vec![2],
        };
        debug_assert_eq!(calculation.multiply(), vec![2, 5, 0])
    }

    #[test]
    fn test_calculate_multiply_6() {
        let calculation = Calculation {
            first_row: vec![1, 2, 1],
            second_row: vec![9],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 0, 8, 9])
    }

    #[test]
    fn test_calculate_multiply_7() {
        let calculation = Calculation {
            first_row: vec![1, 2, 1],
            second_row: vec![9, 0],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 0, 8, 9, 0])
    }
    #[test]
    fn test_calculate_multiply_8() {
        let calculation = Calculation {
            first_row: vec![1, 2, 7],
            second_row: vec![9, 0],
        };
        debug_assert_eq!(calculation.multiply(), vec![1, 1, 4, 3, 0])
    }
}
