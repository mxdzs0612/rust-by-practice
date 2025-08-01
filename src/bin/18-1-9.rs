// Input functions
// Since closure can be used as arguments, you might wonder can we use functions as arguments too? And indeed we can.

// 🌟🌟

// /* Implement `call_me` to make it work */
// fn call_me {
//     f();
// }

// fn function() {
//     println!("I'm a function!");
// }

// fn main() {
//     let closure = || println!("I'm a closure!");

//     call_me(closure);
//     call_me(function);
// }


/* Implement `call_me` to make it work */
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}