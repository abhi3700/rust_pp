//! ```
//! fn main() {
//!     let s = String::from("hello, world");
//!     let word = first_word(&s);
//!     println!("The first word is: {}", word);
//! }
//!
//! fn first_word(s: &String) -> &str {
//!     // Implement this function to return the first word in the string
//! }
//! ```

pub fn main() {
    let s = String::from("hello, world");
    let word = first_word(&s);
    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    // Implement this function to return the first word in the string
    // s.trim().to_string().split(' ').collect()
    s.trim()
        .split(' ')
        .next()
        .expect("the first word should have been present")
}
