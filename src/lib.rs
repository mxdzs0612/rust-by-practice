// // 填空
// // in lib.rs

// mod front_of_house {
//     // 实现此模块
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         pub fn seat_at_table() {}
//     }

//     pub mod serving {
//         pub fn take_order() {}

//         pub fn serve_order() {}

//         pub fn take_payment() {}

//         // 我猜你不希望顾客听到你在抱怨他们，因此让这个函数私有化吧
//         fn complain() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 使用绝对路径调用
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 使用相对路径调用
//     front_of_house::hosting::add_to_waitlist();
// }

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // 使用三种方式填空
//         //1. 使用关键字 `super`
//         //2. 使用绝对路径
//         super::front_of_house::serving::serve_order();
//     }

//     fn cook_order() {}
// }

// in src/lib.rs

pub mod front_of_house;
pub mod back_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}