fn main() {
    let start = std::time::Instant::now();
    // with_capacity()是String类型的构造函数，
    // 用于创建一个新字符串并预分配指定大小的内部缓冲区，
    // 参数10000表示预分配10000字节的容量
    let mut s1 = String::with_capacity(10000);
    for _ in 0..10000 {
        s1.push_str("a");
    }
    println!("耗时: {:?}", start.elapsed());
}
// 运行输出：
// 耗时: 634.042µs
