//! Write a function that takes a vector of strings as an argument
//! and returns a new vector with all the strings
//! that contain the letter 'a'. The original vector should not be modified.

#[allow(dead_code)]
fn filter_strings_with_a(v: &Vec<String>) -> Vec<String> {
    v.iter().filter(|s| s.contains('a')).cloned().collect()
}

#[test]
fn test_empty_vec() {
    let v1 = Vec::<String>::new();
    assert_eq!(filter_strings_with_a(&v1), Vec::<String>::new());
    assert_eq!(v1, Vec::<String>::new());
}

#[test]
fn test_vec_w_elements_no_a() {
    let v1 = vec!["bhi".to_string(), "kevin".to_string()];
    assert_eq!(filter_strings_with_a(&v1), Vec::<String>::new());
    assert_eq!(v1, vec!["bhi".to_string(), "kevin".to_string()]);
}

#[test]
fn test_vec_w_elements_a() {
    let v1 = vec!["abhi".to_string(), "kevin".to_string()];
    assert_eq!(filter_strings_with_a(&v1), vec!["abhi".to_string()]);
    assert_eq!(v1, vec!["abhi".to_string(), "kevin".to_string()]);
}
