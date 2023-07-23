//! Write a function that takes a vector of integers as an argument
//! and returns a new vector with all the even numbers doubled.
//! The original vector should not be modified.

fn vec_double_even(v1: &Vec<u32>) -> Vec<u32> {
    v1.iter().filter(|&x| x % 2 == 0).map(|y| y * 2).collect()
}

#[test]
fn test_w_vec_of_only_odds() {
    let v1 = vec![1, 3, 5];
    assert_eq!(vec_double_even(&v1), Vec::<u32>::new());
    assert_eq!(v1, vec![1, 3, 5]);
}

#[test]
fn test_w_vec_of_odds_evens() {
    let v1 = vec![1, 2, 3, 5];
    assert_eq!(vec_double_even(&v1), vec![4]);
    assert_eq!(v1, vec![1, 2, 3, 5]);
}

#[test]
fn test_w_vec_of_only_evens() {
    let v1 = vec![2, 4, 6];
    assert_eq!(vec_double_even(&v1), vec![4, 8, 12]);
    assert_eq!(v1, vec![2, 4, 6]);
}
