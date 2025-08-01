// 🌟
// /* Fill in the blank */
// fn main() {
//     let mut names = vec!["Bob", "Frank", "Ferris"];

//     for name in names.__{
//         *name = match name {
//             &mut "Ferris" => "There is a rustacean among us!",
//             _ => "Hello",
//         }
//     }

//     println!("names: {:?}", names);
// }

/* Fill in the blank */
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut(){
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}