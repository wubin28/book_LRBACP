fn main() {
    let start = std::time::Instant::now();
    let mut s1 = String::new();
    // 确保字符串至少有10000字节的可用容量，
    // 如果当前容量小于10000，它会重新分配内存，
    // 但不会改变字符串的内容和长度，只增加容量
    s1.reserve(10000);
    for _ in 0..10000 {
        s1.push_str("a");
    }
    println!("耗时: {:?}", start.elapsed());
}
// 运行输出：
// 耗时: 636.667µs
