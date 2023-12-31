/// Finds the intersection of two vectors, duplicates in vector b are preserved.
pub fn intersection<T: PartialEq + Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut result = Vec::new();

    for item in a {
        if b.contains(item) {
            result.push(item.clone());
        }
    }

    result
}