// 🌟🌟 There are a few preserved aliases in Rust, one of which can be used in impl blocks.
// enum VeryVerboseEnumOfThingsToDoWithNumbers {
//     Add,
//     Subtract,
// }

// impl VeryVerboseEnumOfThingsToDoWithNumbers {
//     fn run(&self, x: i32, y: i32) -> i32 {
//         match self {
//             __::Add => x + y,
//             __::Subtract => x - y,
//         }
//     }
// }

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            &Self::Add => x + y,
            &Self::Subtract => x - y,
        }
    }
}

fn main() {}