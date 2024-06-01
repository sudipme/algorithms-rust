# Execution Examples

**Test array**

```rust
let mut array = [2, 5, 7, 9, 10, 13];
```
# Sorting

## merge sort

```rust
let merge_sorted = sorting::merge_sort::sort(&array, 0, array.len()-1);
println!("Sorted using merge sort\n {:?}\n", merge_sorted);
```
## bubble sort

```rust
sorting::bubble_sort::sort(&mut array);
```

# Searching

## binary search

```rust
let vec = Vec::from(&array);
let result = searching::binary_search::search(&array, target);
let result = searching::binary_search::jump_search(&array, target);
```
## Applications of binary search

### Finding the smallest value for which a function is true
```rust
fn test_function(val: i32) -> bool{
val >= 15
}
let solution = searching::binary_search::smallest_function_solution(test_function, 10);
```

### Finding maximum value of a function
```rust
fn test_function(val: i32) -> i32 {
-(val - 4).pow(2)
}
let solution = searching::binary_search::find_maximum_value(test_function, -10, 10);
```
# Complete search

## Generating subsets (recursive method)

```rust
let array = [0, 1, 2];
let mut subset: Vec<i32> = Vec::new();
let mut subset_vector: Vec<Vec<i32>> = Vec::new();
complete_search::generating_subsets::recursive_method(&array, &mut subset, &mut subset_vector, 0);
println!("{:?}", subset_vector);
```