// match, matches! 和 if let
// match
// 🌟🌟

// // 填空
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         __  => { // 在这里匹配 South 或 North
//             println!("South or North");
//         },
//         _ => println!(__),
//     };
// }


// 填空
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { // 在这里匹配 South 或 North
            println!("South or North");
        },
        _ => println!("WEST"),
    };
}