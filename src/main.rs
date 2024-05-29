mod sorting;

fn main() {
    let mut array = [8,7,6,5,4,3,2,1];

    let merge_sorted = sorting::merge_sort::sort(&array, 0, array.len()-1);
    println!("Sorted using merge sort\n {:?}\n", merge_sorted);

    println!("Bubble sort");
    sorting::bubble_sort::sort(&mut array);
}



