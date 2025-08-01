// NLL（非词汇生命周期）(Non-Lexical Lifetime)
// 在解释 NLL 之前，我们先看一段代码：

// fn main() {
//    let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     println!("{} and {}", r1, r2);

//     let r3 = &mut s;
//     println!("{}", r3);
// }
// 根据我们目前的知识，这段代码会因为违反 Rust 中的借用规则而导致错误。

// 但是，如果您执行 cargo run ，那么一切都没问题，那么这里发生了什么？

// 编译器在作用域结束之前判断不再使用引用的能力称为 非词法生命周期（简称 NLL ）。

// 有了这种能力，编译器就知道最后一次使用引用是什么时候，并根据这些知识优化借用规则。

// let mut u = 0i32;
// let mut v = 1i32;
// let mut w = 2i32;

// // lifetime of `a` = α ∪ β ∪ γ
// let mut a = &mut u;     // --+ α. lifetime of `&mut u`  --+ lexical "lifetime" of `&mut u`,`&mut u`, `&mut w` and `a`
// use(a);                 //   |                            |
// *a = 3; // <-----------------+                            |
// ...                     //                                |
// a = &mut v;             // --+ β. lifetime of `&mut v`    |
// use(a);                 //   |                            |
// *a = 4; // <-----------------+                            |
// ...                     //                                |
// a = &mut w;             // --+ γ. lifetime of `&mut w`    |
// use(a);                 //   |                            |
// *a = 5; // <-----------------+ <--------------------------+
// 再借用
// 学习了 NLL 之后，我们现在可以很容易地理解再借用了。

// 示例

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Point {
//     fn move_to(&mut self, x: i32, y: i32) {
//         self.x = x;
//         self.y = y;
//     }
// }

// fn main() {
//     let mut p = Point { x: 0, y: 0 };
//     let r = &mut p;
//     // 这里是再借用
//     let rr: &Point = &*r;

//     println!("{:?}", rr); // 这里结束再借用

//     // 再借用结束，现在我们可以继续使用 `r`
//     r.move_to(10, 10);
//     println!("{:?}", r);
// }
// 🌟🌟
// /* 通过重新排序一些代码使下面代码正常运行 */
// fn main() {
//     let mut data = 10;
//     let ref1 = &mut data;
//     let ref2 = &mut *ref1;

//     *ref1 += 1;
//     *ref2 += 2;

//     println!("{}", data);
// }

/* 通过重新排序一些代码使下面代码正常运行 */
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    *ref2 += 2;
    *ref1 += 1;

    println!("{}", data);
}