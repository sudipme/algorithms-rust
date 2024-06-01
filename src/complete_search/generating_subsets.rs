pub fn recursive_method(slice: &[i32], subset: &mut Vec<i32>, subset_vector: &mut Vec<Vec<i32>>, index: usize) {
    if index == slice.len() {
        // Base case: if the index is equal to the length of the slice,
        // add the current subset to the subset vector
        subset_vector.push(subset.clone())
    } else {
        recursive_method(slice, subset, subset_vector, index + 1);
        subset.push(slice[index]);
        recursive_method(slice, subset, subset_vector, index + 1);
        subset.pop();
    }
}