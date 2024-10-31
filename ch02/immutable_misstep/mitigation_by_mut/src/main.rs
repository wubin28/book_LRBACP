fn main() {
    // 声明了可变变量sum并初始化为0，
    // 也就是将0绑定到可变变量sum上
    let mut sum = 0;
    for i in 1..=3 {
        // sum自增i
        sum += i;
    }
    println!("求和结果: {}\n", sum);
}
// 运行输出:
// 求和结果: 6