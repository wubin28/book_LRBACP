fn main() {
    let x: f64 = 0.1 + 0.2;
    let epsilon: f64 = 1e-10; // 定义一个很小的误差范围
    if (x - 0.3).abs() < epsilon {
        println!("x约等于0.3");
    } else {
        println!("x不约等于0.3");
    }
    println!("x的值是：{}", x);
    println!("程序结束");
}
// 输出：
// x约等于0.3
// x的值是：0.30000000000000004
// 程序结束
