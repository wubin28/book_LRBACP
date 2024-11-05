fn main() {
    let chinese_text = String::from("第2章");

    for ch in chinese_text.chars() {
        println!("{}所占字节数：{}", ch, std::mem::size_of_val(&ch));
    }

    println!("字符数量: {}", chinese_text.chars().count());
}
// 运行输出：
// 第所占字节数：4
// 2所占字节数：4
// 章所占字节数：4
// 字符数量: 3