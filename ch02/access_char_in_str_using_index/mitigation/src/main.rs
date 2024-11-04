fn main() {
    // String::from()是一个静态方法，用于从字符串字面量创建一个String类型
    let string = String::from("你好");

    // string.chars()创建一个迭代器，用于遍历字符串中的每个Unicode字符
    // .next()获取迭代器的第一个元素
    // 由于.next()返回Option类型，使用if let来解构Some值
    // 如果成功获取到第一个字符，它会被绑定到first_char变量
    if let Some(first_char) = string.chars().next() {
        println!("第一个字符是: {}", first_char);
    }
}
// 运行输出：
// 第一个字符是: 你
