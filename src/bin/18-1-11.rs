// 🌟🌟
// /* Fill in the blank and fix the error*/
// fn factory(x:i32) -> __ {

//     let num = 5;

//     if x > 1{
//         move |x| x + num
//     } else {
//         move |x| x + num
//     }
// }

/* Fill in the blank and fix the error*/
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
}

// 下面不是题

// Closure in structs
// Example

struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn main() {}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 1);
}
