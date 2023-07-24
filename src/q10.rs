//! Write a function that takes a vector of strings as an argument and
//! returns a new vector with all the strings sorted in descending order by length.
//! The original vector should not be modified.

fn vec_to_desc(v1: &Vec<String>) -> Vec<String> {
    let mut v2 = v1.to_owned();
    v2.sort_by(|a, b| b.len().cmp(&a.len()));
    v2.to_vec().to_vec()
}

#[test]
fn test_random_vec() {
    let v1 = vec![
        "alice".to_string(),
        "bob".to_string(),
        "charlie".to_string(),
    ];
    assert_eq!(
        vec_to_desc(&v1),
        vec![
            "charlie".to_string(),
            "alice".to_string(),
            "bob".to_string()
        ]
    );
    assert_eq!(
        v1,
        vec![
            "alice".to_string(),
            "bob".to_string(),
            "charlie".to_string(),
        ]
    );
}

#[test]
fn test_desc_sorted_vec() {
    let v1 = vec![
        "charlie".to_string(),
        "alice".to_string(),
        "bob".to_string(),
    ];
    assert_eq!(
        vec_to_desc(&v1),
        vec![
            "charlie".to_string(),
            "alice".to_string(),
            "bob".to_string()
        ]
    );
    assert_eq!(
        v1,
        vec![
            "charlie".to_string(),
            "alice".to_string(),
            "bob".to_string()
        ]
    );
}
