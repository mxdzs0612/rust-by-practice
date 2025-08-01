// Move closures may still implement Fn or FnMut, even though they capture variables by move. This is because the traits implemented by a closure type are determined by what the closure does with captured values, not how it captures them. The move keyword only specifies the latter.

// fn main() {
//     let s = String::new();

//     let update_string = move || println!("{}",s);

//     exec(update_string);
// }

// fn exec<F: FnOnce()>(f: F)  {
//     f()
// }
// The following code also has no error:


// fn main() {
//     let s = String::new();

//     let update_string = move || println!("{}",s);

//     exec(update_string);
// }

// fn exec<F: Fn()>(f: F)  {
//     f()
// }
// 🌟🌟
// /* Fill in the blank */
// fn main() {
//     let mut s = String::new();

//     let update_string = |str| -> String {s.push_str(str); s };

//     exec(update_string);
// }

// fn exec<'a, F: __>(mut f: F) {
//     f("hello");
// }

/* Fill in the blank */
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
    f("hello");
}