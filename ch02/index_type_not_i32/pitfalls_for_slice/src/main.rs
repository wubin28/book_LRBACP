fn main() {
    let array = [10, 20, 30, 40, 50];
    let slice = &array[1..4]; //取得数组的一个切片
    let index: i32 = 1;
    println!("slice[{}] = {}", index, slice[index]);
}
// 编译错误：
// error[E0277]: the type `[{integer}]` cannot be indexed by `i32`
