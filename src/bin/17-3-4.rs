// HRTB（更高等级特征约束）(Higher-ranked trait bounds)
// 类型约束可能在生命周期中排名更高。这些约束指定了一个约束对于所有生命周期都为真。例如，诸如此类的约束 for<'a> &'a T: PartialEq<i32> 需要如下实现：

// impl<'a> PartialEq<i32> for &'a T {
//     // ...
// }
// 然后可以用于将一个 &'a T 与任何生命周期进行比较 i32 。

// 这里只能使用更高级别的约束，因为引用的生命周期比函数上任何可能的生命周期参数都短。

// 🌟🌟🌟
// /* 添加 HRTB 使下面代码正常运行！ */
// fn call_on_ref_zero<'a, F>(f: F) where F: Fn(&'a i32) {
//     let zero = 0;
//     f(&zero);
// }

// fn main() {
//     println!("Success!")
// }

/* 添加 HRTB 使下面代码正常运行！ */
fn call_on_ref_zero<F>(f: F) where F: for<'a> Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

fn main() {
    println!("Success!")
}