fn main() {
    let num = "55".parse().expect("无法解析数字");
    println!("num: {}", num);
}
// 编译错误：
// error[E0282]: type annotations needed