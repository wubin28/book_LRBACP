fn main() {
    let vec = vec![10, 20, 30, 40, 50];
    let index: i32 = 3;
    println!("vec[{}] = {}", index, vec[index]);
}
// 编译错误：
// error[E0277]: the type `[{integer}]` cannot be indexed by `i32`
