use crate::value::Value;

fn check_is_normalised(vector: &Vec<u8>) -> bool {
    vector.iter().all(|entry| entry < &10)
}

fn normalise_step(vector: Vec<u8>) -> Vec<u8> {
    let mut cloned_vec = vector.clone();
    for (index, entry) in cloned_vec.clone().iter().enumerate() {
        if entry >= &10 {
            let digit = cloned_vec[index] / 10;
            cloned_vec[index] = entry % 10;
            if index == 0 {
                cloned_vec.insert(0, digit);
            } else {
                cloned_vec[index - 1] += digit
            }
        }
    }
    cloned_vec
}

pub fn normalise_vector(vector: Vec<Value>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for entry in vector.iter() {
        let current_entry = result.get(entry.col_sum);
        if current_entry.is_some() {
            result[entry.col_sum] += entry.value
        } else {
            result.push(entry.value)
        }
    }
    let mut normalised = check_is_normalised(&result);
    while !normalised {
        result = normalise_step(result);
        normalised = check_is_normalised(&result);
    }
    result
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_normalise_10() {
        let vector = multiply_vectors(vec![3], vec![4]);
        debug_assert_eq!(normalise_vector(vector), vec![1, 2]);
    }

    #[test]
    fn test_normalise_11() {
        let vector = multiply_vectors(vec![2], vec![3]);
        debug_assert_eq!(normalise_vector(vector), vec![6]);
    }
}
