// 🌟🌟 Slice is unsized type, but the reference of slice is not.
// /* Make it work with slice references */
// fn main() {
//     let s: str = "Hello there!";

//     let arr: [u8] = [1, 2, 3];
// }

/* Make it work with slice references */
fn main() {
    let s: &str = "Hello there!";

    let arr: &[u8] = &[1, 2, 3];
}