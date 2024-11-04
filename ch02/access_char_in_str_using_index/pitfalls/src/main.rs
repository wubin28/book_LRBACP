fn main() {
    let string = String::from("你好");
    let ch = string[0];
    println!("第一个字符是: {}", ch);
}
// 编译错误：
// error[E0277]: the type `String` cannot be indexed by `{integer}`
