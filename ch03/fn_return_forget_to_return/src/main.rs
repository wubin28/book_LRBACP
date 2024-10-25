fn main() {
    let result = check_and_double(15);
    println!("结果: {}", result);
}

fn check_and_double(x: i32) -> i32 {
    if x > 10 {
        x * 2 // 这里缺少return关键字
    }
    x // 函数会继续执行到这里
}
// 编译错误
// error[E0308]: mismatched types
//  --> src/main.rs:8:9
//   |
// 7 | /     if x > 10 {
// 8 | |         x * 2 // 这里缺少return关键字
//   | |         ^^^^^ expected `()`, found `i32`
// 9 | |     }
//   | |_____- expected this to be `()`
//   |
// help: consider using a semicolon here
//   |
// 9 |     };
//   |      +
// help: you might have meant to return this value
//   |
// 8 |         return x * 2; // 这里缺少return关键字
//   |         ++++++      +