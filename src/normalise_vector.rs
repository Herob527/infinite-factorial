use crate::value::Value;

pub fn normalise_vector(vector: Vec<Value>) -> Vec<u8> {
    if vector.iter().all(|entry| entry.value < 10) {
        return vector.iter().map(|entry| entry.value).collect();
    };
    let mut is_normalised = false;
    let mut vector_copy: Vec<u8> = vector.clone().iter().map(|x| x.value).collect();
    while !is_normalised {
        let mut remember: u8 = 0;
        vector_copy.reverse();
        is_normalised = true;
        let reversed_version = vector_copy.iter();
        let mut resulting_vector: Vec<u8> = Vec::new();
        for entry in reversed_version {
            let current_entry = entry.to_owned() + remember;
            if current_entry >= 10 {
                remember = (f32::from(current_entry) / f32::from(10 as u8)).floor() as u8;
                let remainder = current_entry.to_owned() % 10 as u8;
                is_normalised = false;
                resulting_vector.insert(0, remainder);
                continue;
            }
            resulting_vector.insert(0, current_entry);
        }
        if remember > 0 {
            resulting_vector.insert(0, remember);
            is_normalised = true;
        }
        dbg!(&resulting_vector);
        if is_normalised {
            return resulting_vector;
        }
    }
    return vec![];
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
}
