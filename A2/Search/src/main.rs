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

/*
(5 points) Provide a compelling argument (aka a proof) of correctness, perhaps involving invariants. Problem 6.28 can help with understanding this terminology, if you took CMPT260 rather than CMPT263.
- Invariant: At the start of each loop iteration, if the target exists in the vector, it is contained within vector[low...high].
- Initialization: Initially, low = 0 and high = vector.len() - 1, so the entire vector is considered, and the invariant holds.
- Maintenance: Each iteration calculates the mid-point:
    - If vector[mid] equals the target, the function returns mid.
    - If the target is greater than vector[mid], then low is updated to mid + 1.
    - If the target is less, high is updated to mid - 1.
    - In both updates, if the target was present, it remains in the new subarray defined by the updated low and high values.
- Termination: When low > high, the subarray is empty, meaning the target is not in the vector, and the function returns -1.
*/

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

/*
(5 points) A functional version using recursion and cannot involve mutable variables.
- Base Case: When high < low, the sub-array is empty and the function correctly returns -1.
- Inductive Step: Assume the function works correctly for all sub-arrays smaller than a given size.
- For a sub-array of size n:
    - The mid-element is checked:
        - If it equals the target, the correct index is returned.
        - If not, the function makes a recursive call on a smaller sub-array (either left or right of mid).
    - By the induction hypothesis, the recursive call correctly returns the index if the target exists, or -1 if it does not.
- Conclusion: By mathematical induction, the recursive binary search function is correct.
*/

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

/*
(5 points) Which program was easier to write, and why? Which proof was easier to write, and why? In keeping with the theory concept informing this course, your answer may relate to the unwritten culture of programming.
- The imperative version is generally easier to write since loops and mutable variables are more intuitive.
- Its correctness proof via loop invariants is straightforward as you directly update the search boundaries.
- The functional (recursive) version, involves recursion and induction, making its proof slightly more abstract.
*/