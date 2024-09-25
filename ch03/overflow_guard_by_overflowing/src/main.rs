fn main() {
    let x: u8 = 255;
    println!("Initial value: {}", x);

    // 演示溢出问题并使用overflowing_add处理
    let (result, overflowed) = x.overflowing_add(1);
    println!("After adding 1: {}", result);
    println!("Did overflow occur? {}", overflowed);

    // 使用overflowing_add方法处理正常加法
    let y: u8 = 10;
    let (result, overflowed) = y.overflowing_add(1);
    println!("10 + 1 = {}", result);
    println!("Did overflow occur? {}", overflowed);
}
// 输出：
// Initial value: 255
// After adding 1: 0
// Did overflow occur? true
// 10 + 1 = 11
// Did overflow occur? false