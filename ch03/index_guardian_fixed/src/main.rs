use std::io;

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    println!("请输入一个索引：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");

    match input.trim().parse::<usize>() {
        Ok(index) if index < numbers.len() => {
            println!("索引 {} 的元素是: {}", index, numbers[index]);
        }
        Ok(index) => {
            println!("索引 {} 超出数组范围，数组长度为 {}", index, numbers.len());
        }
        Err(_) => {
            println!("无效的输入，请输入一个非负整数");
        }
    }
}
// 输出：
// 请输入一个索引：
// 10
// 索引 10 超出数组范围，数组长度为 5
