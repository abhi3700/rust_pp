//! **Custom Fibonacci iterator**: Implement a custom iterator that generates Fibonacci numbers. Test your iterator by generating the first 10 Fibonacci numbers.

struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;
        let new_curr = std::mem::replace(&mut self.next, new_next);

        self.current = new_curr;
        Some(new_curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 0,
        next: 1,
    }
}

#[test]
fn test_fibonacci() {
    let fib = fibonacci();
    for i in fib.take(10) {
        println!("{}", i);
    }
}
