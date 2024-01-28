#[derive(Debug, Clone)]
pub struct Value {
    pub value: u8,
    pub first_col: usize,
    pub second_col: usize,
    pub col_sum: usize,
}
