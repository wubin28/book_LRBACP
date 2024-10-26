fn main() {
    let number = 5;

    // 错误：期望bool类型，得到i32类型
    if number {
        println!("数字是真的？");
    }
    println!("程序结束");
}
// 编译错误：
// error[E0308]: mismatched types
//  --> src/main.rs:5:8
//   |
// 5 |     if number {
//   |        ^^^^^^ expected `bool`, found integer
