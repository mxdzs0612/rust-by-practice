// Closure as return types
// Returning a closure is much harder than you may have thought of.

// 🌟🌟
// /* Fill in the blank using two approaches,
//  and fix the error */
// fn create_fn() -> __ {
//     let num = 5;

//     // How does the following closure capture the environment variable `num`
//     // &T, &mut T, T ?
//     |x| x + num
// }


// fn main() {
//     let fn_plain = create_fn();
//     fn_plain(1);
// }

/* Fill in the blank using two approaches,
 and fix the error */
fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    move |x| x + num
}


fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}