// 结构体
// 三种类型的结构体
// 🌟 对于结构体，我们必须为其中的每一个字段都指定具体的值

// // fix the error
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }
// fn main() {
//     let age = 30;
//     let p = Person {
//         name: String::from("sunface"),
//         age,
//     };
// } 


// fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "".to_string(),
    };
} 