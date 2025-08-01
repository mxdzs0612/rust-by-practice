// 未约束的生命周期
// 在 Nomicon - Unbounded Lifetimes 中查看更多信息。

// 更多省略规则
// impl<'a> Reader for BufReader<'a> {
//     // 'a 在以下方法中不使用
// }

// // 可以写为：
// impl Reader for BufReader<'_> {
    
// }
// // Rust 2015
// struct Ref<'a, T: 'a> {
//     field: &'a T
// }

// // Rust 2018
// struct Ref<'a, T> {
//     field: &'a T
// }
// 艰难的练习
// 🌟🌟🌟🌟
// /* 使下面代码正常运行 */
// struct Interface<'a> {
//     manager: &'a mut Manager<'a>
// }

// impl<'a> Interface<'a> {
//     pub fn noop(self) {
//         println!("interface consumed");
//     }
// }

// struct Manager<'a> {
//     text: &'a str
// }

// struct List<'a> {
//     manager: Manager<'a>,
// }

// impl<'a> List<'a> {
//     pub fn get_interface(&'a mut self) -> Interface {
//         Interface {
//             manager: &mut self.manager
//         }
//     }
// }

// fn main() {
//     let mut list = List {
//         manager: Manager {
//             text: "hello"
//         }
//     };

//     list.get_interface().noop();

//     println!("Interface should be dropped here and the borrow released");

//     use_list(&list);
// }

// fn use_list(list: &List) {
//     println!("{}", list.manager.text);
// }
struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
    where 'a: 'b {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn main() {

    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

// Claude 的方案
// use std::cell::RefCell;
// struct Manager<'a> {
//     text: &'a str
// }
// struct Interface<'a> {
//     manager: &'a RefCell<Manager<'a>>
// }
// impl<'a> Interface<'a> {
//     pub fn noop(self) {
//         println!("interface consumed");
//     }
// }
// struct List<'a> {
//     manager: RefCell<Manager<'a>>,
// }
// impl<'a> List<'a> {
//     pub fn get_interface(&'a self) -> Interface<'a> {
//         Interface {
//             manager: &self.manager
//         }
//     }
// }
// fn main() {
//     let list = List {
//         manager: RefCell::new(Manager {
//             text: "hello"
//         })
//     };
//     list.get_interface().noop();
//     println!("Interface should be dropped here and the borrow released");
//     use_list(&list);
// }
// fn use_list(list: &List) {
//     println!("{}", list.manager.borrow().text);
// }
