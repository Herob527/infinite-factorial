use crate::value::Value;

fn check_is_normalised(vector: &Vec<u8>) -> bool {
    vector.iter().all(|entry| entry < &10)
}

pub fn normalise_vector(vector: Vec<Value>) -> Vec<u8> {
    if vector.iter().all(|entry| entry.value < 10) {
        return vector.iter().map(|entry| entry.value).collect();
    };

    let mut result: Vec<u8> = Vec::new();

    for (index, entry) in vector.iter().enumerate() {
        if Some(&0) == result.get(index) {
            result[index] += entry.value
        } else {
            result.insert(0, entry.value)
        }
    }

    return result;
}

#[cfg(test)]
mod tests2 {
    use crate::multiply_vectors::multiply_vectors;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_normalise() {
        let vector = multiply_vectors(vec![1, 2, 1], vec![9]);
        debug_assert_eq!(normalise_vector(vector), vec![1, 0, 8, 9]);
    }

    #[test]
    fn test_normalise_2() {
        let vector = multiply_vectors(vec![1, 1], vec![9]);
        debug_assert_eq!(normalise_vector(vector), vec![9, 9]);
    }

    #[test]
    fn test_normalise_3() {
        let vector = multiply_vectors(vec![1, 1], vec![1, 0]);
        debug_assert_eq!(normalise_vector(vector), vec![1, 1, 0]);
    }

    #[test]
    fn test_normalise_4() {
        let vector = multiply_vectors(vec![1], vec![1, 0]);
        debug_assert_eq!(normalise_vector(vector), vec![1, 0]);
    }

    #[test]
    fn test_normalise_5() {
        let vector = multiply_vectors(vec![2], vec![1, 0]);
        debug_assert_eq!(normalise_vector(vector), vec![2, 0]);
    }

    #[test]
    fn test_normalise_6() {
        let vector = multiply_vectors(vec![2], vec![5, 0]);
        debug_assert_eq!(normalise_vector(vector), vec![1, 0, 0]);
    }
    #[test]
    fn test_normalise_7() {
        let vector = multiply_vectors(vec![2], vec![2]);
        debug_assert_eq!(normalise_vector(vector), vec![4]);
    }

    #[test]
    fn test_normalise_8() {
        let vector = multiply_vectors(vec![1, 4], vec![1, 2]);
        debug_assert_eq!(normalise_vector(vector), vec![1, 6, 8]);
    }

    #[test]
    fn test_normalise_9() {
        let vector = multiply_vectors(vec![1, 2, 1], vec![1, 0]);
        debug_assert_eq!(normalise_vector(vector), vec![1, 2, 1, 0]);
    }
}
