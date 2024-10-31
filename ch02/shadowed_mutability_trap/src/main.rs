fn main() {
    let mut count = 0;

    let count = count + 1;

    count += 1;
}
// 编译错误：
// error[E0384]: cannot assign twice to immutable variable `count`