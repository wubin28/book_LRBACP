fn main() {
    // 整型变量默认推断为i32
    let integer = 42;
    println!("integer的类型是: {}", std::any::type_name_of_val(&integer));

    // 使用usize作为集合类型的索引
    let array = [1, 2, 3, 4, 5];
    let index: usize = 2;
    println!("array[{}] = {}", index, array[index]);

    // 浮点型变量默认推断为f64
    let float = 3.14;
    println!("float的类型是: {}", std::any::type_name_of_val(&float));

    // 显示usize的大小
    println!(
        "在这个系统上, usize的大小是: {} 位",
        std::mem::size_of::<usize>() * 8
    );
}
// 输出：
// integer的类型是: i32
// array[2] = 3
// float的类型是: f64
// 在这个系统上, usize的大小是: 64 位
