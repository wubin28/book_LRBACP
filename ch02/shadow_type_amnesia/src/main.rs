fn main() {
    let x = "55";

    let x = x.parse::<i32>().unwrap();

    println!("x: {}", x.len());
}
// 编译错误：
// error[E0599]: no method named `len` found for type `i32` in the current scope
