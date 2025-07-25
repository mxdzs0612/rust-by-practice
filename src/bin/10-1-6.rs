// 方法
// 🌟🌟🌟
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // 实现 mixup，不要修改其它代码！
//     fn mixup
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// }
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // 实现 mixup，不要修改其它代码！
    fn mixup<V, W>(self, p2: Point<V, W>) -> Point<T, W> {
        Point { x: (self.x), y: (p2.y) }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}