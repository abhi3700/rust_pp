//! Generate collatz sequence/conjecture for a given number
//! For example, let's start with n=6:
//!
//! 6 is even, so divide by 2: 6/2 = 3
//! 3 is odd, so multiply by 3 and add 1: 3*3 + 1 = 10
//! 10 is even, so divide by 2: 10/2 = 5
//! 5 is odd, so multiply by 3 and add 1: 3*5 + 1 = 16
//! 16 is even, so divide by 2: 16/2 = 8
//! 8 is even, so divide by 2: 8/2 = 4
//! 4 is even, so divide by 2: 4/2 = 2
//! 2 is even, so divide by 2: 2/2 = 1
//!
//! The sequence for n=6 is: 6, 3, 10, 5, 16, 8, 4, 2, 1.

fn generate_collatz(num: u32) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    let mut x = num;
    res.push(x);

    loop {
        if x == 1 {
            break;
        }

        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = (3 * x + 1) / 2
        }

        res.push(x);
    }
    res
}

#[test]
fn test_collatz() {
    let n = 5;
    assert_eq!(vec![5, 8, 4, 2, 1], generate_collatz(n));
}
