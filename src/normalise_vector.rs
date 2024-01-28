pub fn normalise_vector(vector: Vec<Value>) -> Vec<Value> {
    if vector.iter().all(|entry| entry.value < 10) {
        return vector;
    };
    return vec![];
}
