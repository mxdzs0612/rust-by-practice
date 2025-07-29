// Deref 特征
// Deref 特征在智能指针 - Deref章节 https://practice.rs/smart-pointers/deref.html 中有更加详细的介绍。

// transmute
// std::mem::transmute 是一个 unsafe 函数，可以把一个类型按位解释为另一个类型，其中这两个类型必须有同样的位数( bits )。

// transmute 相当于将一个类型按位移动到另一个类型，它会将源值的所有位拷贝到目标值中，然后遗忘源值。该函数跟 C 语言中的 memcpy 函数类似。

// 正因为此，transmute 非常非常不安全! 调用者必须要自己保证代码的安全性，当然这也是 unsafe 的目的。

// 示例
// transmute 可以将一个指针转换成一个函数指针，该转换并不具备可移植性，原因是在不同机器上，函数指针和数据指针可能有不同的位数( size )。
// fn foo() -> i32 {
//     0
// }

// fn main() {
//     let pointer = foo as *const ();
//     let function = unsafe {
//         std::mem::transmute::<*const (), fn() -> i32>(pointer)
//     };
//     assert_eq!(function(), 0);
// }
// transmute 还可以扩展或缩短一个不变量的生命周期，将 Unsafe Rust 的不安全性体现的淋漓尽致!
// struct R<'a>(&'a i32);
// unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
//     std::mem::transmute::<R<'b>, R<'static>>(r)
// }

// unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>)
//                                              -> &'b mut R<'c> {
//     std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
// }
// 事实上我们还可以使用一些安全的方法来替代 transmute.
fn main() {
    /*Turning raw bytes(&[u8]) to u32, f64, etc.: */
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    let num = unsafe { std::mem::transmute::<[u8; 4], u32>(raw_bytes) };

    // use `u32::from_ne_bytes` instead
    let num = u32::from_ne_bytes(raw_bytes);
    // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
    let num = u32::from_le_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);
    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);

    /*Turning a pointer into a usize: */
    let ptr = &0;
    let ptr_num_transmute = unsafe { std::mem::transmute::<&i32, usize>(ptr) };

    // Use an `as` cast instead
    let ptr_num_cast = ptr as *const i32 as usize;

    /*Turning an &mut T into an &mut U: */
    let ptr = &mut 0;
    let val_transmuted = unsafe { std::mem::transmute::<&mut i32, &mut u32>(ptr) };

    // Now, put together `as` and reborrowing - note the chaining of `as`
    // `as` is not transitive
    let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };

    /*Turning an &str into a &[u8]: */
    // this is not a good way to do this.
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    assert_eq!(slice, &[82, 117, 115, 116]);

    // You could use `str::as_bytes`
    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);

    // Or, just use a byte string, if you have control over the string
    // literal
    assert_eq!(b"Rust", &[82, 117, 115, 116]);
}
// 没有题