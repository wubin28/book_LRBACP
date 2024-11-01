fn main() {
    let number = 7;

    let result = if number < 5 {
        "less than five"
    } else if number > 10 {
        "greater than ten"
    };

    println!("The result is: {}", result);
}
// 编译错误：
// error[E0317]: `if` may be missing an `else` clause
