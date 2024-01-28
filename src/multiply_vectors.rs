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
            result.insert(0, value);
        }
    }
    return result;
}
