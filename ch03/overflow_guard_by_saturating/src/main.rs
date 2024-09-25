fn main() {
    let x: u8 = 255;
    println!("Initial value: {}", x);

    // 演示溢出问题并使用saturating_add处理
    let result = x.saturating_add(1);
    println!("After saturating_add 1: {}", result);

    // 再次使用saturating_add
    let result = result.saturating_add(1);
    println!("After saturating_add 1 again: {}", result);

    // 使用saturating_add方法处理正常加法
    let y: u8 = 10;
    let result = y.saturating_add(1);
    println!("10 + 1 = {}", result);
}
// 输出：
// Initial value: 255
// After saturating_add 1: 255
// After saturating_add 1 again: 255
// 10 + 1 = 11