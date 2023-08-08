//! **Find the maximum**: Write a function that takes an iterator
//! over integers and returns the maximum value.
//! Test your function with a range of numbers and a vector of integers.

fn get_max<I>(itr: &mut I) -> u32
where
    I: Iterator<Item = u32>,
{
    let mut max = itr.next().unwrap_or(0);
    println!("max initiated with: {}", max);
    for i in itr {
        println!("{}", i);
        if i > max {
            max = i;
            println!("{}", max);
        }
    }
    max
}

#[test]
fn test_max() {
    let mut itr = 5..=10;
    assert_eq!(get_max(&mut itr), 10);
}
