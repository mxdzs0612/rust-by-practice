// 🌟🌟🌟
// #[derive(Debug, PartialEq)]
// struct EvenNum(i32);

// impl TryFrom<i32> for EvenNum {
//     type Error = ();

//     // 实现 `try_from`
//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNum(value))
//         } else {
//             Err(())
//         }
//     }
// }

// fn main() {
//     assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
//     assert_eq!(EvenNum::try_from(5), Err(()));

//     // 填空
//     let result: Result<EvenNum, ()> = 8i32.try_into();
//     assert_eq!(result, __);
//     let result: Result<EvenNum, ()> = 5i32.try_into();
//     assert_eq!(result, __);

//     println!("Success!")
// }

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // 实现 `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // 填空
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, EvenNum::try_from(8));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, EvenNum::try_from(5));

    println!("Success!")
}

// fn main() {
//     assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
//     assert_eq!(EvenNum::try_from(5), Err(()));

//     let result: Result<EvenNum, ()> = 8i32.try_into();
//     assert_eq!(result, Ok(EvenNum(8)));
//     let result: Result<EvenNum, ()> = 5i32.try_into();
//     assert_eq!(result, Err(()));
// }