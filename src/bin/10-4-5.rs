// 对象安全
// 一个特征能变成特征对象，首先该特征必须是对象安全的，即该特征的所有方法都必须拥有以下特点：

// 返回类型不能是 Self.
// 不能使用泛型参数
// 🌟🌟🌟🌟

// // 使用至少两种方法让代码工作
// // 不要添加/删除任何代码行
// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Self { 42 }
// }

// impl MyTrait for String {
//     fn f(&self) -> Self { self.clone() }
// }

// fn my_function(x: Box<dyn MyTrait>)  {
//     x.f()
// }

// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));

//     println!("Success!")
// }

// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!")
}

// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> u32 { 42 }
// }

// impl MyTrait for String {
//     fn f(&self) -> String { self.clone() }
// }

// fn my_function(x: impl MyTrait) -> impl MyTrait  {
//     x.f()
// }

// fn main() {
//     my_function(13_u32);
//     my_function(String::from("abc"));
// }