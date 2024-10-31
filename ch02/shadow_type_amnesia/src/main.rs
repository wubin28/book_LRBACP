fn main() {
    // 将字符串字面量"55"绑定给x
    // x的类型是&str（字符串切片）
    let x = "55";

    // 将x从&str类型转换为i32类型的整数，
    // parse()方法会返回Result类型，
    // unwrap()用于获取成功情况下的值
    let x = x.parse::<i32>().unwrap();

    // 尝试打印字符串切片x的长度
    println!("x: {}", x.len());
}
// 编译错误：
// error[E0599]: no method named `len` found for type `i32` in the current scope
