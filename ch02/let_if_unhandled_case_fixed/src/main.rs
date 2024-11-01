fn main() {
    let number = 7;

    let result = if number < 5 {
        "less than five"
    } else if number > 10 {
        "greater than ten"
    } else {
        "between five and ten"
    };

    println!("The result is: {}", result);
}
// 运行输出：
// The result is: between five and ten
