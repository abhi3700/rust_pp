pub(crate) fn main() {
    let s1 = String::from("hello");
    println!("original string: {s1}");
    let s2 = clone_string(s1);
    println!("cloned string: {s2}");
}

fn clone_string(s: String) -> String {
    // TODO: Implement this function to return a clone of the input string
    s.clone()
}
