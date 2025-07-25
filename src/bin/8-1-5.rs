// 🌟🌟

// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if e == MyEnum::Foo { // 修复错误，只能修改本行代码
//             count += 1;
//         }
//     }

//     assert_eq!(count, 2);
// }

enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { // 修复错误，只能修改本行代码
            count += 1;
        }
    }

    assert_eq!(count, 2);
}
