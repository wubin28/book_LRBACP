fn main() {
    // 整数除法
    println!("5 / 2 = {}", 5 / 2); // 输出: 2

    // 字符型（char）
    let character: char = 'B';
    println!("字符: {}", character);

    // 字符串字面量（&str，即字符串切片）
    let string_slice: &str = "Hello, world!";
    println!("字符串切片: {}", string_slice);

    // String类型
    let mut growable_string: String = String::from("Hello");
    growable_string.push_str(", Rust!");
    println!("可增长的字符串: {}", growable_string);

    // 数值字面量与类型后缀
    let unsigned_8bit: u8 = 55u8;
    let float_32bit: f32 = 1.0f32;
    println!("8位无符号整数: {}", unsigned_8bit);
    println!("32位浮点数: {}", float_32bit);
}
// 输出：
// 5 / 2 = 2
// 字符: B
// 字符串切片: Hello, world!
// 可增长的字符串: Hello, Rust!
// 8位无符号整数: 55
// 32位浮点数: 1
