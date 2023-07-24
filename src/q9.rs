//! Write a function that takes a vector of integers as an argument
//! and returns a new vector with all the prime numbers in the
//! original vector. The original vector should not be modified.

fn is_prime(num: &u32) -> bool {
    if *num < 2 {
        return false;
    }
    for i in 2..*num {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn vec_of_int_to_prime(v1: &Vec<u32>) -> Vec<u32> {
    v1.iter()
        .filter(|&x| is_prime(x))
        .cloned()
        .collect::<Vec<u32>>()
}

#[test]
fn test_some_vec_elem_below_2() {
    let v1 = vec![1, 4, 6, 7, 9, 11];
    assert_eq!(vec_of_int_to_prime(&v1), vec![7, 11]);
    assert_eq!(v1, vec![1, 4, 6, 7, 9, 11])
}

#[test]
fn test_some_vec_elem_including_2() {
    let v1 = vec![2, 4, 6, 7, 9, 11];
    assert_eq!(vec_of_int_to_prime(&v1), vec![2, 7, 11]);
    assert_eq!(v1, vec![2, 4, 6, 7, 9, 11])
}

#[test]
fn test_some_vec_elem_above_2() {
    let v1 = vec![3, 4, 6, 7, 9, 11];
    assert_eq!(vec_of_int_to_prime(&v1), vec![3, 7, 11]);
    assert_eq!(v1, vec![3, 4, 6, 7, 9, 11])
}
