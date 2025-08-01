// 🌟🌟🌟🌟 使用 Box::leak 也可以产生 'static 生命周期
// #[derive(Debug)]
// struct Config {
//     a: String,
//     b: String,
// }
// static mut config: Option<&mut Config> = None;

// /* 让代码工作，但不要修改函数的签名 */
// fn init() -> Option<&'static mut Config> {
//     Some(&mut Config {
//         a: "A".to_string(),
//         b: "B".to_string(),
//     })
// }

// fn main() {
//     unsafe {
//         config = init();

//         println!("{:?}",config)
//     }
// }

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* 让代码工作，但不要修改函数的签名 */
// https://www.zhihu.com/question/511520023/answer/2310578784
fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    Some(Box::leak(c))
}

#[allow(static_mut_refs)]
fn main() {
    unsafe {
        config = init();

        println!("{:?}", config)
    }
}
