// O(n log n)
pub fn sort(arr: &[i32], f:usize, l:usize) -> Vec<i32> {
    if f == l {
        println!("End of recursion return:  {}",arr[f]);
        return vec![arr[f]];
    }
    println!("Array: {:?}, Front: {}, Back: {}", &arr[f..l+1], f, l);
    let k = (f+l)/2;
    let larr = sort(arr, f, k);
    let rarr = sort(arr, k+1, l);
    println!("left array: {:?}, right array: {:?}", larr, rarr);
    merge(&larr, &rarr)
}

pub fn merge(larr: &Vec<i32>, rarr:&Vec<i32>) -> Vec<i32>{
    let l_len = larr.len();
    let r_len = rarr.len();
    let mut vec:Vec<i32> = Vec::new();

    let mut l = 0;
    let mut r = 0;

    while l < l_len && r < r_len {

        if larr[l] < rarr[r] {
            vec.push(larr[l]);
            l+=1;
        } else {
            vec.push(rarr[r]);
            r += 1;
        }
    }
    if l_len > l {
        for i in &larr[l..] {
            vec.push(*i);
        }
    }

    if r_len > r {
        for i in &rarr[r..] {
            vec.push(*i);
        }
    }
    println!("Merged array: {:?}\n", vec);
    vec
}
