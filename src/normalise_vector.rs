use crate::value::Value;

pub fn normalise_vector(vector: Vec<Value>) -> Vec<u8> {
    if vector.iter().all(|entry| entry.value < 10) {
        return vector.iter().map(|entry| entry.value).collect();
    };
    dbg!(vector);
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
}
