// Fn, FnMut, FnOnce
// When taking a closure as an input parameter, the closure's complete type must be annotated using one of the following traits:

// Fn: the closure uses the captured value by reference (&T)
// FnMut: the closure uses the captured value by mutable reference (&mut T)
// FnOnce: the closure uses the captured value by value (T)
// 🌟🌟
// /* Make it work by changing the trait bound, in two ways*/
// fn fn_once<F>(func: F)
// where
//     F: FnOnce(usize) -> bool,
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     let x = vec![1, 2, 3];
//     fn_once(|z|{z == x.len()})
// }

/* Make it work by changing the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}

// fn fn_once<F>(func: F)
// where
//     F: Copy + FnOnce(usize) -> bool,// 改动在这里
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     let x = vec![1, 2, 3];
//     fn_once(|z|{z == x.len()})
// }