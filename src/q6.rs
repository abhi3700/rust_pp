//! Write a function that takes a vector of strings as an argument
//! and returns a new vector with all the strings in uppercase.
//! The original vector should not be modified.

fn uppercase_str(v: &Vec<String>) -> Vec<String> {
    v.iter().map(|s| s.to_uppercase()).collect()
}

#[test]
fn test_empty_str() {
    let v1 = vec![];
    assert_eq!(uppercase_str(&v1), Vec::<String>::new());
    assert_eq!(v1, Vec::<String>::new());
}

#[test]
fn test_some_str() {
    let v1_str = vec!["alice", "bob"];
    let v1 = v1_str.iter().map(|&s| String::from(s)).collect();
    assert_eq!(
        uppercase_str(&v1),
        vec!["ALICE".to_string(), "BOB".to_string()]
    );
    assert_eq!(v1, vec!["alice".to_string(), "bob".to_string()]);
}

#[test]
fn test_some_string() {
    let v1 = vec!["alice".to_string(), "bob".to_string()];
    assert_eq!(
        uppercase_str(&v1),
        vec!["ALICE".to_string(), "BOB".to_string()]
    );
    assert_eq!(v1, vec!["alice".to_string(), "bob".to_string()]);
}
