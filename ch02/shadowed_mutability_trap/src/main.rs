fn main() {
    let mut count = 0;
    println!("初始 count: {}", count);

    // 遮蔽为不可变
    let count = count + 1;
    println!("遮蔽后的 count: {}", count);

    // 试图修改 count，会导致编译错误
    count += 1; // 错误：count 是不可变的
}
// 编译错误：
// error[E0384]: cannot assign twice to immutable variable `count`
// --> src/main.rs:10:5
// |
// 6  |     let count = count + 1;
// |         ----- first assignment to `count`
// ...
// 10 |     count += 1; // 错误：count 是不可变的
// |     ^^^^^^^^^^ cannot assign twice to immutable variable
