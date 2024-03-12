use crate::value::Value;

pub fn multiply_vectors(vec1: Vec<u64>, vec2: Vec<u64>) -> Vec<Value> {
    let mut result: Vec<Value> = Vec::new();
    for (index_first_row, entry_first_row) in vec1.iter().enumerate() {
        for (index_second_row, entry_second_row) in vec2.iter().enumerate() {
            let multiplication = entry_first_row * entry_second_row;
            let col_sum = index_first_row + index_second_row;
            let value = Value {
                value: multiplication,
                col_sum,
            };
            result.push(value);
        }
    }
    result
}

#[cfg(test)]
mod tests3 {
    use crate::multiply_vectors::multiply_vectors;

    #[test]
    fn test_multiply_vectors() {
        let vector: Vec<u64> = multiply_vectors(vec![1, 2, 1], vec![9])
            .iter()
            .map(|x| x.value)
            .collect();
        debug_assert_eq!(vector, vec![9, 18, 9]);
    }

    #[test]
    fn test_multiply_vectors2() {
        let vector: Vec<u64> = multiply_vectors(vec![1], vec![1, 0])
            .iter()
            .map(|x| x.value)
            .collect();
        debug_assert_eq!(vector, vec![1, 0]);
    }
}
