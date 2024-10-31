fn main() {
    let value = 55;
    println!("value: {}", value);

    let value = value.to_string();
    println!("value: {}", value);

    let value = value.len();
    println!("value: {}", value);
}
// 运行输出：
// value: 55
// value: 55
// value: 2
