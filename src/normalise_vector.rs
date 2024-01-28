use crate::value::Value;

pub fn normalise_vector(vector: Vec<Value>) -> Vec<u8> {
    if vector.iter().all(|entry| entry.value < 10) {
        return vector.iter().map(|entry| entry.value).collect();
    };
    let mut is_normalised = false;
    let mut vector_copy: Vec<u8> = vector.clone().iter().map(|x| x.value).collect();
    while !is_normalised {
        let mut remember: u8 = 0;
        is_normalised = true;
        let reversed_version = vector_copy.iter_mut().rev();
        for (index, entry) in reversed_version.enumerate() {
            if entry >= &mut 10 {
                remember = (f32::from(entry.to_owned()) / f32::from(10 as u8)).floor() as u8;
                let remainder = entry.to_owned() % 10 as u8;
                is_normalised = false;
            }
        }
        if remember > 0 {}
    }
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
