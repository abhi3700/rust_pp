//! Write a function that takes a string slice as an argument
//! and returns a new string with all the characters reversed.
//! The original string should not be modified.

fn reverse_str(s: &str) -> String {
    // M-1: Manual
    // let mut res = String::new();
    // for i in 0..s.len() {
    //     res.push(s.chars().nth(s.len() - 1 - i).unwrap_or_default());
    // }
    // res

    // M-2: Using Idioms
    s.chars().rev().collect()
}

#[test]
fn test_some_str() {
    let s1 = "hello";
    assert_eq!(reverse_str(s1), "olleh".to_string());
    assert_eq!(s1, "hello");
}

#[test]
fn test_some_string() {
    let s0 = String::from("hello");
    let s1 = &s0;
    assert_eq!(reverse_str(s1), "olleh".to_string());
    assert_eq!(s1, "hello");
}
