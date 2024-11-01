fn main() {
    let condition1 = true;
    let condition2 = false;

    let result = match (condition1, condition2) {
        (true, true) => "both true",
        (true, false) => "only condition1 true",
        (false, true) => "only condition2 true",
        (false, false) => "both false",
    };

    println!("The result is: {}", result);
}
// 运行输出：
// The result is: only condition1 true
