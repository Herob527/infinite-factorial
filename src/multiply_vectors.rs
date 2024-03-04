use crate::value::Value;

pub fn multiply_vectors(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<Value> {
    let mut result: Vec<Value> = Vec::new();
    for (index_first_row, entry_first_row) in vec1.iter().enumerate() {
        for (index_second_row, entry_second_row) in vec2.iter().enumerate() {
            let multiplication = entry_first_row * entry_second_row;
            let col_sum = index_first_row + index_second_row;
            let value = Value {
                value: multiplication,
                first_col: index_first_row,
                second_col: index_second_row,
                col_sum,
            };
            result.push(value);
        }
    }
    let values: Vec<u8> = result.iter().map(|x| x.value).collect();
    println!("Result: {:?}", values);
    result
}

#[cfg(test)]
mod tests3 {
    use crate::multiply_vectors::multiply_vectors;

    #[test]
    fn test_multiply_vectors() {
        let vector: Vec<u8> = multiply_vectors(vec![1, 2, 1], vec![9])
            .iter()
            .map(|x| x.value)
            .collect();
        debug_assert_eq!(vector, vec![9, 18, 9]);
    }

    #[test]
    fn test_multiply_vectors2() {
        let vector: Vec<u8> = multiply_vectors(vec![1], vec![1, 0])
            .iter()
            .map(|x| x.value)
            .collect();
        debug_assert_eq!(vector, vec![1, 0]);
    }
}
