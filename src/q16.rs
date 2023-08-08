//! **String concatenation**:
//! Write a function that takes an iterator over strings and
//! returns a single concatenated string. Test your function
//! with a vector of strings and an iterator that yields strings from a file.

fn concat_strings<I>(itr: I) -> String
where
    // NOTE: Here, instead of `String` type, we should make this
    // more generic to allow any string slices w/o allowing only String objects.
    I: Iterator<Item = &'static str>,
{
    let mut concatenated_string = String::new();
    for i in itr {
        concatenated_string.push_str(i);
    }

    concatenated_string
}

#[test]
fn test_concat_string() {
    let s1 = vec!["abc", "def", "ghi"];

    assert_eq!(concat_strings(s1.into_iter()), "abcdefghi".to_string());
}
