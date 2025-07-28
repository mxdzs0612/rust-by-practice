// 静态分发和动态分发Static and Dynamic dispatch
// 关于这块内容的解析介绍，请参见 Rust语言圣经 https://course.rs/basic/trait/trait-object.html#%E7%89%B9%E5%BE%81%E5%AF%B9%E8%B1%A1%E7%9A%84%E5%8A%A8%E6%80%81%E5%88%86%E5%8F%91。

// 🌟🌟

// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String { format!("u8: {}", *self) }
// }

// impl Foo for String {
//     fn method(&self) -> String { format!("string: {}", *self) }
// }

// // 通过泛型实现以下函数
// fn static_dispatch...

// // 通过特征对象实现以下函数
// fn dynamic_dispatch...

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success!")
// }

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// 通过泛型实现以下函数
fn static_dispatch<T: Foo>(x: T) {
    println!("{}", x.method());
}

// 通过特征对象实现以下函数
fn dynamic_dispatch(x: &dyn Foo) {
    println!("{}", x.method());
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!")
}
