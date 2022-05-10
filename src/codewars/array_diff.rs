pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    // When filtering a vector, don't iterate over references.
    // Use the actual values.
    a.into_iter().filter(|x| !b.contains(x)).collect()
}
