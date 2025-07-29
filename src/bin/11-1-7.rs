// 🌟🌟🌟

// // 填空
// use std::mem;

// fn main() {
//     let story = String::from("Rust By Practice");

//     // 阻止 String 的数据被自动 drop
//     let mut story = mem::ManuallyDrop::new(story);

//     let ptr = story.__();
//     let len = story.__();
//     let capacity = story.__();

//     assert_eq!(16, len);

//     // 我们可以基于 ptr 指针、长度和容量来重新构建 String. 
//     // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
//     let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

//     assert_eq!(*story, s);

//     println!("Success!")
// }


// 填空
use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    // 阻止 String 的数据被自动 drop
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    // 我们可以基于 ptr 指针、长度和容量来重新构建 String. 
    // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}