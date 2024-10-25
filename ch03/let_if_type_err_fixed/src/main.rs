fn main() {
    let condition = true;

    // 修复后的示例
    let number = if condition {
        5
    } else {
        6 // 修改为整数类型,与if分支保持一致
    };

    println!("The value of number is: {}", number);
}
// 输出：
// The value of number is: 5
