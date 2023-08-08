//! *Filter even numbers*:
//! Write a function that takes an iterator over integers and
//! returns a new iterator that yields only the even numbers.
//! Test your function with a range of numbers and a vector of integers.

use std::vec;

fn filter_even<I>(itr: I) -> impl Iterator<Item = u32>
where
    I: Iterator<Item = u32>,
{
    itr.filter(|x| x % 2 == 0)
}

pub fn main() {}

#[test]
fn test_filter_even() {
    // range
    let r = 0..5;
    assert_eq!(filter_even(r).collect::<Vec<u32>>(), vec![0, 2, 4]);
    let v1 = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(
        filter_even(v1.into_iter()).collect::<Vec<u32>>(),
        vec![0, 2, 4]
    );
}
