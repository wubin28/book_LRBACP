fn main() {
    let condition1 = true;
    let condition2 = false;

    // 过于复杂的示例
    let result = if condition1 {
        if condition2 {
            "both true"
        } else {
            "only condition1 true"
        }
    } else {
        if condition2 {
            "only condition2 true"
        } else {
            "both false"
        }
    };

    println!("The result is: {}", result);
}
// 输出：
// The result is: only condition1 true
