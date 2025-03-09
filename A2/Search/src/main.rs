// An imperative version (as described in the book) using
// a loop and mutable variables
fn binary_search_imperative(vector: &Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = (vector.len() as i32) - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        // If target is present at mid
        if vector[mid as usize] == target {
            return mid;
        }

        // If target is greater
        if vector[mid as usize] < target {
            low = mid + 1;
        }

        // If target is smaller
        else {
            high =  mid - 1;
        }
    }
    -1
}

// A functional version using recursion and cannot involve
// mutable variables
fn binary_search_functional(vector: &Vec<i32>, target: i32, low: i32, high: i32) -> i32 {
    if high >= low {
        let mid = low + (high - low) / 2;

        // If target is present at mid
        if vector[mid as usize] == target {
            return mid;
        }

        // If target is greater
        if vector[mid as usize] > target {
            return binary_search_functional(vector, target, low, mid - 1);
        }

        // If target is less
        else {
            return binary_search_functional(vector, target, mid + 1, high);
        }
    }
    -1
}

fn main() {
    let sorted_vector = vec![1, 2, 13, 20, 30, 31, 42, 44, 77, 98];

    // Test case where target is present
    let target = 77;
    let result = binary_search_imperative(&sorted_vector, target);
    if result == -1 {
        println!("Binary search (imperative): target {} was not found in the vector.", target);
    } else {
        println!("Binary search (imperative): target {} found at index {}.", target, result);
    }

    // Test cases where target is not present
    let target = 15;
    let result = binary_search_imperative(&sorted_vector, target);
    if result == -1 {
        println!("Binary search (imperative): target {} was not found in the vector.", target);
    } else {
        println!("Binary search (imperative): target {} found at index {}.", target, result);
    }

    // Test case where target is present
    let target = 98;
    let result = binary_search_functional(&sorted_vector, target, 0, (sorted_vector.len() as i32) - 1);
    if result == -1 {
        println!("Binary search (functional): target {} was not found in the vector.", target);
    } else {
        println!("Binary search (functional): target {} found at index {}.", target, result);
    }

    // Test cases where target is not present
    let target = 3;
    let result = binary_search_functional(&sorted_vector, target, 0, (sorted_vector.len() as i32) - 1);
    if result == -1 {
        println!("Binary search (functional): target {} was not found in the vector.", target);
    } else {
        println!("Binary search (functional): target {} found at index {}.", target, result);
    }
}
