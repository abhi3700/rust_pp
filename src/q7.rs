//! **Sum of squares**:
//! Write a function that takes an iterator over integers
//! and returns the sum of their squares. Test your function with a range of
//! numbers and a vector of integers.

fn sum_of_iter_of_integers<I: Iterator<Item = u32>>(iter: I) -> u32 {
    iter.map(|x| x.pow(2)).collect::<Vec<_>>().iter().sum()
}

#[test]
fn test_some_range() {
    let iter = 1..=6;
    assert_eq!(
        sum_of_iter_of_integers(iter),
        1u32.pow(2) + 2u32.pow(2) + 3u32.pow(2) + 4u32.pow(2) + 5u32.pow(2) + 6u32.pow(2)
    );
}

#[test]
fn test_some_vec() {
    let v1: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(
        sum_of_iter_of_integers(v1.iter().cloned()),
        1u32.pow(2) + 2u32.pow(2) + 3u32.pow(2) + 4u32.pow(2) + 5u32.pow(2) + 6u32.pow(2)
    );
}
