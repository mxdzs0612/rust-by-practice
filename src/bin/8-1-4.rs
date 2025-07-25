// matches!
// matches! 看起来像 match, 但是它可以做一些特别的事情

// 🌟🌟

// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // 使用 `matches!` 填空
//     for ab in alphabets {
//         assert!(__)
//     }
// } 


fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // 使用 `matches!` 填空
    for ab in alphabets {
        assert!(matches!(ab, '0'..='z'))
    }
} 

// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // fill the blank with `matches!` to make the code work
//     for ab in alphabets {
//         assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
//     }
// } 