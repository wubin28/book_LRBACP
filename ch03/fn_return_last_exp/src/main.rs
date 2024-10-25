fn main() {
    let result = calculate_sum(5, 3);
    println!("计算结果: {}", result);
}

fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b; // 这里误加了分号
}
// 编译错误：
// error[E0308]: mismatched types
//  --> src/main.rs:6:37
//   |
// 6 | fn calculate_sum(a: i32, b: i32) -> i32 {
//   |    -------------                    ^^^ expected `i32`, found `()`
//   |    |
//   |    implicitly returns `()` as its body has no tail or `return` expression
// 7 |     a + b; // 这里误加了分号
//   |          - help: remove this semicolon to return this value