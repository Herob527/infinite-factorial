use crate::{multiply_vectors::multiply_vectors, normalise_vector::normalise_vector, value::Value};

#[derive(Debug, Clone)]
pub struct Calculation {
    pub first_row: Vec<u8>,
    pub second_row: Vec<u8>,
}

impl Calculation {
    pub fn multiply(&self) -> Vec<u8> {
        let result = multiply_vectors(self.clone().first_row, self.clone().second_row);
        // dbg!(&result);
        let normalised_vector = normalise_vector(result);
        // dbg!(&normalised_vector);
        return normalised_vector;
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
