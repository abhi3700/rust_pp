pub(crate) fn main() {
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);
}

fn append_world(s: &mut String) {
    // TODO: Implement this function to append ", world" to the string

    s.push_str(", world")
}
