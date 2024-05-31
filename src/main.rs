mod searching;
mod sorting;

fn main() {
    println!("algorithms-rust\n");
}

// Execution Examples

// test array
// let mut array = [2, 5, 7, 9, 10, 13];

// merge sort
// let merge_sorted = sorting::merge_sort::sort(&array, 0, array.len()-1);
// println!("Sorted using merge sort\n {:?}\n", merge_sorted);

//bubble sort
// println!("Bubble sort");
// sorting::bubble_sort::sort(&mut array);

// binary search
// let vec = Vec::from(&array);
// let result = searching::binary_search::search(&array, target);
// let result = searching::binary_search::jump_search(&array, target);
//
// Applications of binary search
//
// Finding the smallest value for which a function is true
// fn test_function(val: i32) -> bool{
//     val >= 15
// }
// let solution = searching::binary_search::smallest_function_solution(test_function, 10);
//
// Finding maximum value of a function
// fn test_function(val: i32) -> i32 {
//     -(val - 4).pow(2)
// }
// let solution = searching::binary_search::find_maximum_value(test_function, -10, 10);