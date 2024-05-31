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

// we wish to find the smallest value that is a valid solution for a problem.
// We are given a function test_function(x) that returns true if x is a valid solution and false otherwise.
pub fn smallest_function_solution(function: fn(i32) -> bool, initial_jump: i32) -> i32 {
    // The initial jump length has to be large enough,
    // for example some value for which we know beforehand that test_function(jump) is true.
    let mut jump = initial_jump;
    let mut x = -1;

    while jump >= 1 {
        println!("previous jump{jump}");
        while !function(x + jump) {
            x += jump;
            println!("jumped to: {x}");
        }
        jump /= 2;
        println!("new jump {jump}");
    }
    // The search finds the largest value of x for which the function(x) is false.
    // Thus, the next value x + 1 is the smallest possible value for which function(x) is true.
    x + 1
}

// starting point should be smaller than position of max value
// the function returns the x value of the function for which the function produce max value.
pub fn find_maximum_value(function: fn(i32) -> i32, starting_point: i32, initial_jump: i32) -> i32 {
    let mut jump = initial_jump;
    let mut x = starting_point;

    while jump >= 1 {
        println!("previous jump{jump}");
        while function(x + jump) < function(x + jump + 1){
            x += jump;
            println!("jumped to: {x}");
        }
        jump /= 2;
        println!("new jump {jump}");
    }
    x + 1
}


