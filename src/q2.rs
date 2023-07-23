//! Write a function that takes a string slice as an argument and
//! returns a new string with all the vowels removed.
//! The original string should not be modified.

fn str_to_vowels(s: &str) -> String {
    s.chars()
        .filter(|&c| !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'))
        .collect()
}

#[test]
fn test_lowercase_only() {
    let slice = "hello";
    assert_eq!(str_to_vowels(slice), "hll");
    assert_eq!(slice, "hello");
}

#[test]
fn test_uppercase_only() {
    let slice = "hEllO";
    assert_eq!(str_to_vowels(slice), "hll");
    assert_eq!(slice, "hEllO");
}

#[test]
fn test_lowercase_uppercase() {
    let slice = "hEllo";
    assert_eq!(str_to_vowels(slice), "hll");
    assert_eq!(slice, "hEllo");
}
