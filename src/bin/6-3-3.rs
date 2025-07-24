// 🌟🌟
// fn main() {
//    let arr: [i32; 5] = [1, 2, 3, 4, 5];
//   // 填空让代码工作起来
//   let slice: __ = __;
//   assert_eq!(slice, &[2, 3, 4]);
// }


fn main() {
   let arr: [i32; 5] = [1, 2, 3, 4, 5];
  // 填空让代码工作起来
  let slice: &[i32] = &arr[1..=3];
  assert_eq!(slice, &[2, 3, 4]);
}