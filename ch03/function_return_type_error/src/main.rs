fn main() {
    let result = get_status(true);
    println!("状态: {}", result);
}

fn get_status(flag: bool) -> String {
    if flag {
        "OK".to_string()
    } else {
        404 // 这里返回了一个数字，而不是字符串
    }
}
// 编译错误：
// error[E0308]: mismatched types
//   --> src/main.rs:10:9
//    |
// 6  | fn get_status(flag: bool) -> String {
//    |                              ------ expected `String` because of return type
// ...
// 10 |         404 // 这里返回了一个数字，而不是字符串
//    |         ^^^- help: try using a conversion method: `.to_string()`
//    |         |
//    |         expected `String`, found integer
