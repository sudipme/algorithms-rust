// At each step, the search checks the middle element of the active region.
// If the middle element is the target element, the search terminates.
// Otherwise, the search recursively continues to the left or right half of the region,
// depending on the value of the middle element.
// time complexity: O(log n)
pub fn search(slice: &[i32], target: i32) -> i32 {
    if slice.is_empty() {
        return -1;
    }

    let mut front = 0;
    let mut back = slice.len() - 1;

    while front <= back {
        let mid = (front + back) / 2;
        if slice[mid] == target {
            return mid as i32;
        } else if slice[mid] < target {
            front = mid + 1;
        } else {
            // Using wrapping_sub to handle potential underflow as "back" is usize and can't be negative
            back = back.wrapping_sub(1);
        }
    }

    -1
}

// left to right binary search using jumps
// idea is to make jumps and slow the speed when we get closer to the target element.
// time complexity: O(log n)
pub fn jump_search(slice: &[i32], target: i32) -> i32 {
    let len = slice.len();
    let mut head = 0;
    let mut jump = len / 2;

    // if the jump is 0, that means either element is found or does not exist.
    while jump >= 1 {
        // if after jump target is still ahead of head, keep moving the head according to previous jump.
        // if after jump the new head is going out of bound,
        // or the target will be behind the new head, reduce the jump unit by factor of 2.
        println!("previous head:{head}, previous jump{jump}");
        while head + jump < len && slice[head + jump] <= target {
            head += jump;
            println!("head jumped to: {head}");
        }
        jump /= 2;
        println!("new jump {jump}");
    }

    if slice[head] == target {
        return head as i32;
    }

    -1
}
