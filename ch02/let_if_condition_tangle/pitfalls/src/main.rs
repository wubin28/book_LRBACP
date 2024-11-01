fn main() {
    let condition1 = true;
    let condition2 = false;

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
// 运行输出：
// The result is: only condition1 true
