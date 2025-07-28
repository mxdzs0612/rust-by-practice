// Traits 这玩意一般叫特质，但《圣经》这本书给翻译成了特征，这里就不改了
// 特征 Trait 可以告诉编译器一个特定的类型所具有的、且能跟其它类型共享的特性。我们可以使用特征通过抽象的方式来定义这种共享行为，还可以使用特征约束来限定一个泛型类型必须要具有某个特定的行为。

// Note: 特征跟其它语言的接口较为类似，但是仍然有一些区别

// 示例


// struct Sheep { naked: bool, name: String }

// impl Sheep {
//     fn is_naked(&self) -> bool {
//         self.naked
//     }

//     fn shear(&mut self) {
//         if self.is_naked() {
//             // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
//             println!("{} is already naked...", self.name());
//         } else {
//             println!("{} gets a haircut!", self.name);

//             self.naked = true;
//         }
//     }
// }


// trait Animal {
//     // 关联函数签名；`Self` 指代实现者的类型
//     // 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，这里的 `Self` 指代的就是 `Pig` 类型
//     fn new(name: String) -> Self;

//     // 方法签名
//     fn name(&self) -> String;
    
//     fn noise(&self) -> String;

//     // 方法还能提供默认的定义实现
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }

// impl Animal for Sheep {
//     // `Self` 被替换成具体的实现者类型： `Sheep`
//     fn new(name: String) -> Sheep {
//         Sheep { name: name, naked: false }
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn noise(&self) -> String {
//         if self.is_naked() {
//             "baaaaah?".to_string()
//         } else {
//             "baaaaah!".to_string()
//         }
//     }
    
//     // 默认的特征方法可以被重写
//     fn talk(&self) {
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }

// fn main() {
//     // 这里的类型注释时必须的
//     let mut dolly: Sheep = Animal::new("Dolly".to_string());
//     // TODO ^ 尝试去除类型注释，看看会发生什么

//     dolly.talk();
//     dolly.shear();
//     dolly.talk();
// }
// Exercises
// 🌟🌟


// // 完成两个 `impl` 语句块
// // 不要修改 `main` 中的代码
// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }

//     fn say_something(&self) -> String;
// }

// struct Student {}
// impl Hello for Student {
// }
// struct Teacher {}
// impl Hello for Teacher {
// }

// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");

//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");

//     println!("Success!")
// }


// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}
struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }

    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

fn main() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!")
}