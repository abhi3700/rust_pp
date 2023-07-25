//! Using `.fold()`, fibonacci recursion/iteration

use std::collections::HashMap;

/// Find out the product of elements in a vector.
/// Ensure the original vector is not changed.
fn multiply_vec(v1: &Vec<u32>) -> u32 {
    v1.iter().fold(1, |acc, x| acc * x)
}

#[test]
fn test_1() {
    let v1: Vec<u32> = vec![1, 2, 3, 4];
    assert_eq!(multiply_vec(&v1), 24);
    assert_eq!(v1, vec![1, 2, 3, 4]);
}

/// Find the sum of the even no.s in a vector.
/// Ensure the original vector is not changed.
fn sum_of_evens(v1: &Vec<u32>) -> u32 {
    v1.iter().filter(|&&x| x % 2 == 0).fold(0, |acc, x| acc + x)
}

#[test]
fn test_2() {
    let v1: Vec<u32> = vec![1, 2, 3, 4];
    assert_eq!(sum_of_evens(&v1), 6);
    assert_eq!(v1, vec![1, 2, 3, 4]);
}

/// Write a function that takes an integer n as input and
/// returns the nth Fibonacci number. Use a recursive algorithm to implement the function.

fn fibonacci_recursive(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

#[test]
fn test_3() {
    assert_eq!(fibonacci_recursive(6), 8);
}

/// Write a function that takes an integer n as input
/// and returns the nth Fibonacci number. Use an iterative algorithm to implement the function.
fn fibonacci_iterative(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    // [0, 1, 1, 2, 3, 5, 8, 13, 21]
    //  ˆ  ˆ
    //     ˆ  ˆ
    //        ˆ  ˆ
    //           ˆ  ˆ

    let mut prev_prev = 0;
    let mut prev = 1;

    let mut res = 0; // can be initialized to any no.

    for _ in 2..=n {
        res = prev + prev_prev;
        prev_prev = prev;
        prev = res;
    }

    res
}

#[test]
fn test_4() {
    assert_eq!(fibonacci_iterative(8), 21);
}

#[cfg(test)]
mod test_fibonacci_memoized_iterative {
    use std::collections::HashMap;

    /// Write a function that takes an integer n as input
    /// and returns the nth Fibonacci number. Use memoization technique to implement the function.
    fn fibonacci_memoized_iterative(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
        if let Some(v) = memo.get(&n) {
            return v.to_owned();
        }

        let mut prev_prev = 0;
        memo.insert(0, 0);
        let mut prev = 1;
        memo.insert(1, 1);

        let mut res = 0; // can be initialized to any no.

        println!("Calculated for n: {n}");
        for i in 2..=n {
            res = prev + prev_prev;
            prev_prev = prev;
            prev = res;
            memo.insert(i, res);
        }

        res
    }

    #[test]
    fn test_empty_vec_w_higher_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        assert_eq!(fibonacci_memoized_iterative(8, &mut memo), 21);
        assert_eq!(fibonacci_memoized_iterative(6, &mut memo), 8); // doesn't need calcualation
    }

    #[test]
    fn test_empty_vec_w_lower_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        assert_eq!(fibonacci_memoized_iterative(6, &mut memo), 8);
        assert_eq!(fibonacci_memoized_iterative(8, &mut memo), 21);
    }

    #[test]
    fn test_partial_vec_w_higher_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        memo.insert(0, 0);
        memo.insert(1, 0);
        assert_eq!(fibonacci_memoized_iterative(0, &mut memo), 0);
        assert_eq!(fibonacci_memoized_iterative(8, &mut memo), 21);
        assert_eq!(fibonacci_memoized_iterative(6, &mut memo), 8);
    }

    #[test]
    fn test_partial_vec_w_lower_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        memo.insert(0, 0);
        memo.insert(1, 0);
        assert_eq!(fibonacci_memoized_iterative(0, &mut memo), 0);
        assert_eq!(fibonacci_memoized_iterative(6, &mut memo), 8);
        assert_eq!(fibonacci_memoized_iterative(8, &mut memo), 21);
    }
}

#[cfg(test)]
mod test_fibonacci_memoized_recursive {
    use std::collections::HashMap;

    /// Write a function that takes an integer n as input
    /// and returns the nth Fibonacci number. Use memoization technique to implement the function.
    fn fibonacci_memoized_recursive(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
        // println!("panicked here 1");
        if let Some(v) = memo.get(&n) {
            // println!("No Calculation for 6: {n}");
            return v.to_owned();
        }
        // println!("panicked here 2");

        let res = if n == 0 {
            // println!("panicked here 3");
            0
        } else if n == 1 {
            // println!("panicked here 4");
            1
        } else {
            // println!("panicked here 5");
            fibonacci_memoized_recursive(n - 1, memo) + fibonacci_memoized_recursive(n - 2, memo)
        };

        memo.insert(n, res);

        res
    }

    #[test]
    fn test_empty_map_w_higher_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        assert_eq!(fibonacci_memoized_recursive(8, &mut memo), 21);
        assert_eq!(fibonacci_memoized_recursive(6, &mut memo), 8);
    }

    #[test]
    fn test_empty_map_w_lower_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        assert_eq!(fibonacci_memoized_recursive(6, &mut memo), 8);
        assert_eq!(fibonacci_memoized_recursive(8, &mut memo), 21);
    }

    #[test]
    #[should_panic(expected = "Hashmap should have 1 len with (0,0)")]
    fn test_partial_map_w_higher_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        memo.insert(0, 0);
        memo.insert(1, 0);
        assert_eq!(fibonacci_memoized_recursive(0, &mut memo), 0);
        assert!(memo.len() == 0 || (memo.len() == 1 && memo.get(&0).unwrap().to_owned() == 0));
        assert_eq!(fibonacci_memoized_recursive(8, &mut memo), 21);
        assert_eq!(fibonacci_memoized_recursive(6, &mut memo), 8);
    }

    #[test]
    fn test_1sized_map_w_higher_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        memo.insert(0, 0);
        assert_eq!(fibonacci_memoized_recursive(0, &mut memo), 0);
        assert_eq!(fibonacci_memoized_recursive(8, &mut memo), 21);
        assert_eq!(fibonacci_memoized_recursive(6, &mut memo), 8);
    }

    #[test]
    #[should_panic(expected = "Hashmap should have 1 len with (0,0)")]
    fn test_partial_map_w_lower_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        memo.insert(0, 0);
        memo.insert(1, 0);
        assert_eq!(fibonacci_memoized_recursive(0, &mut memo), 0);
        assert!(memo.len() == 0 || (memo.len() == 1 && memo.get(&0).unwrap().to_owned() == 0));
        assert_eq!(fibonacci_memoized_recursive(6, &mut memo), 8);
        assert_eq!(fibonacci_memoized_recursive(8, &mut memo), 21);
    }

    #[test]
    fn test_1sized_map_w_lower_index_first() {
        let mut memo = HashMap::<u32, u32>::new();
        memo.insert(0, 0);
        assert_eq!(fibonacci_memoized_recursive(0, &mut memo), 0);
        assert_eq!(fibonacci_memoized_recursive(6, &mut memo), 8);
        assert_eq!(fibonacci_memoized_recursive(8, &mut memo), 21);
    }
}
