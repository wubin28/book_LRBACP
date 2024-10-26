fn main() {
    let number = 7;

    // 错误示例
    let result = if number < 5 {
        "less than five"
    } else if number > 10 {
        "greater than ten"
    };

    println!("The result is: {}", result);
}
// 编译错误：
// error[E0317]: `if` may be missing an `else` clause
//  --> main.rs:7:12
//   |
// 7 |       } else if number > 10 {
//   |  ____________^
// 8 | |         "greater than ten"
//   | |         ------------------ found here
// 9 | |     };
//   | |_____^ expected `()`, found `&str`
//   |
//   = note: `if` expressions without `else` evaluate to `()`
//   = help: consider adding an `else` block that evaluates to the expected type
