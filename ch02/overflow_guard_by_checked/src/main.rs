fn main() {
    let x: u8 = 255;
    println!("Initial value: {}", x);

    // 演示溢出问题
    let result = x.checked_add(1);
    match result {
        Some(value) => println!("After adding 1: {}", value),
        None => println!("Overflow occurred!"),
    }

    // 使用checked_add方法处理正常加法
    let y: u8 = 10;
    let result = y.checked_add(1);
    match result {
        Some(value) => println!("10 + 1 = {}", value),
        None => println!("Overflow occurred!"),
    }
}
// 输出：
// Initial value: 255
// Overflow occurred!
// 10 + 1 = 11