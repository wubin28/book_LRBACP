fn main() {
    let start = std::time::Instant::now();
    let mut s1 = String::with_capacity(10000);
    for _ in 0..10000 {
        s1.push_str("a");
    }
    println!("耗时: {:?}", start.elapsed());
}
// 运行输出：
// 耗时: 634.042µs
