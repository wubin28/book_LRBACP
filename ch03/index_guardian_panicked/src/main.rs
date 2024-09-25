use std::io;

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    println!("请输入一个索引：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let index: usize = input.trim().parse().expect("请输入一个有效的数字");

    println!("尝试访问索引 {} 的元素", index);
    let element = numbers[index];
    println!("索引 {} 的元素是: {}", index, element);
}
// 输出：
// 请输入一个索引：
// 10
// 尝试访问索引 10 的元素
// thread 'main' panicked at src/main.rs:12:19:
// index out of bounds: the len is 5 but the index is 10
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
