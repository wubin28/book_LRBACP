fn main() {
    let condition = true;

    // 使用if表达式为变量number绑定值，
    // 如果condition为true，则number绑定5，否则绑定"six"
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
// 编译错误：
// error[E0308]: `if` and `else` have incompatible types
