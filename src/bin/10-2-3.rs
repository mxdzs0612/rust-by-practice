// 🌟🌟🌟 有时我们希望能限制一个变量占用内存的大小，例如在嵌入式环境中，此时 const 泛型参数的第三种形式 const 表达式 就非常适合.
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }

// // 修复 main 函数中的错误
// fn main() {
//     check_size([0u8; 767]); 
//     check_size([0i32; 191]);
//     check_size(["hello你好"; __]); // size of &str ?
//     check_size([(); __].map(|_| "hello你好".to_string()));  // size of String?
//     check_size(['中'; __]); // size of char ?
// }



// pub enum Assert<const CHECK: bool> {}

// pub trait IsTrue {}

// impl IsTrue for Assert<true> {}

#![allow(incomplete_features)]
// 需要 nightly 版本
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// 修复 main 函数中的错误
fn main() {
    check_size([0u8; 767]); 
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // &str 类型，每个 16 字节， N * 16 < 768
    check_size([(); 31].map(|_| "hello你好".to_string())); // String 类型，每个 24 字节， N * 24 < 768
    check_size(['中'; 191]); // char 类型，每个 4 字节，N * 4 < 768
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}