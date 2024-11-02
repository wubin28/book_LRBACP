fn main() {
    let array = [10, 20, 30, 40, 50];
    let index: i32 = 2;
    println!("array[{}] = {}", index, array[index]);
}
// 编译错误：
// error[E0277]: the type `[{integer}]` cannot be indexed by `i32`
