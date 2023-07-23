//! Write a function that takes a vector of integers as an argument
//! and returns the sum of all the even numbers in the vector.
//! The function should not modify the original vector.

#[allow(dead_code)]
fn sum_of_evens(v: &Vec<i32>) -> i32 {
    if !v.is_empty() {
        v.iter().filter(|&i| i % 2 == 0).sum()
    } else {
        0
    }
}

#[test]
fn test_empty_vec() {
    let v1 = Vec::<i32>::new();
    let sum = sum_of_evens(&v1);
    assert_eq!(sum, 0);
    assert_eq!(v1, vec![]);
}

#[test]
fn test_non_empty_vec() {
    let v1 = vec![1, 2, 4, 9];
    assert_eq!(sum_of_evens(&v1), 6);
    assert_eq!(v1, vec![1, 2, 4, 9]);
}
