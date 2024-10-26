fn main() {
    let condition = true;

    // 错误示例
    let number = if condition {
        5
    } else {
        "six" // 这里使用了字符串,与if分支的整数类型不匹配
    };

    println!("The value of number is: {}", number);
}
// 编译错误：
// error[E0308]: `if` and `else` have incompatible types
//  --> main.rs:8:9
//   |
// 5 |       let number = if condition {
//   |  __________________-
// 6 | |         5
//   | |         - expected because of this
// 7 | |     } else {
// 8 | |         "six" // 这里使用了字符串,与if分支的整数类型不匹配
//   | |         ^^^^^ expected integer, found `&str`
// 9 | |     };
//   | |_____- `if` and `else` have incompatible types
