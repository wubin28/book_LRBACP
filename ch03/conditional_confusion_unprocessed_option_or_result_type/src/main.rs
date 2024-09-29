fn main() {
    let maybe_number: Option<i32> = Some(5);
    // 错误：Option<T>不能直接用作条件
    if maybe_number {
        println!("有数字");
    }
    println!("程序结束");
}
// 编译错误：
// error[E0308]: mismatched types
//  --> src/main.rs:4:8
//   |
// 4 |     if maybe_number {
//   |        ^^^^^^^^^^^^ expected `bool`, found `Option<i32>`
//   |
//   = note: expected type `bool`
//              found enum `Option<i32>`
// help: use `Option::is_some` to test if the `Option` has a value
//   |
// 4 |     if maybe_number.is_some() {
//   |                    ++++++++++
