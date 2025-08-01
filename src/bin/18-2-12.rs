// Using closures in iterator adaptors
// 🌟🌟
// /* Fill in the blanks */
// #[derive(PartialEq, Debug)]
// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes.into_iter().__.collect()
// }

// fn main() {
//     let shoes = vec![
//         Shoe {
//             size: 10,
//             style: String::from("sneaker"),
//         },
//         Shoe {
//             size: 13,
//             style: String::from("sandal"),
//         },
//         Shoe {
//             size: 10,
//             style: String::from("boot"),
//         },
//     ];

//     let in_my_size = shoes_in_size(shoes, 10);

//     assert_eq!(
//         in_my_size,
//         vec![
//             Shoe {
//                 size: 10,
//                 style: String::from("sneaker")
//             },
//             Shoe {
//                 size: 10,
//                 style: String::from("boot")
//             },
//         ]
//     );
// }


/* Fill in the blanks */
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}