// 🌟🌟
// /* Make it work in two ways, none of them is to remove `take(movable)` away from the code
// */
// fn main() {
//      let movable = Box::new(3);

//      let consume = || {
//          println!("`movable`: {:?}", movable);
//          take(movable);
//      };

//      consume();
//      consume();
// }

// fn take<T>(_v: T) {}

// For comparison, the following code has no error:


// fn main() {
//      let movable = Box::new(3);

//      let consume = move || {
//          println!("`movable`: {:?}", movable);
//      };

//      consume();
//      consume();
// }





/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
fn main() {
     let movable = Box::new(3);

     let consume = || {
         println!("`movable`: {:?}", movable);
         take(&movable);
     };

     consume();
     consume(); // 法1：注释掉这行
}

fn take<T>(_v: &T) {}