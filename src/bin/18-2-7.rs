// Creating our own iterator
// We can not only create iterators from collection's types, but also can create iterators by implementing the Iterator trait on our own types.

// Example


// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new() -> Counter {
//         Counter { count: 0 }
//     }
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < 5 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// fn main() {
//     let mut counter = Counter::new();

//     assert_eq!(counter.next(), Some(1));
//     assert_eq!(counter.next(), Some(2));
//     assert_eq!(counter.next(), Some(3));
//     assert_eq!(counter.next(), Some(4));
//     assert_eq!(counter.next(), Some(5));
//     assert_eq!(counter.next(), None);
// }
// 🌟🌟🌟
// struct Fibonacci {
//     curr: u32,
//     next: u32,
// }

// // Implement `Iterator` for `Fibonacci`.
// // The `Iterator` trait only requires a method to be defined for the `next` element.
// impl Iterator for Fibonacci {
//     // We can refer to this type using Self::Item
//     type Item = u32;
    
//     /* Implement next method */
//     fn next(&mut self)
// }

// // Returns a Fibonacci sequence generator
// fn fibonacci() -> Fibonacci {
//     Fibonacci { curr: 0, next: 1 }
// }

// fn main() {
//     let mut fib = fibonacci();
//     assert_eq!(fib.next(), Some(1));
//     assert_eq!(fib.next(), Some(1));
//     assert_eq!(fib.next(), Some(2));
//     assert_eq!(fib.next(), Some(3));
//     assert_eq!(fib.next(), Some(5));
// }

struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;
    
    /* Implement next method */
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.curr + self.next;
        self.curr = self.next;
        self.next = res;
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
}