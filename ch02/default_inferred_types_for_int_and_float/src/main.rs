fn main() {
    let integer = 55;
    println!(
        "55的默认推断类型是: {}",
        std::any::type_name_of_val(&integer)
    );

    let float = 3.14;
    println!(
        "3.14的默认推断类型是: {}",
        std::any::type_name_of_val(&float)
    );
}
// 运行输出：
// 55的默认推断类型是: i32
// 3.14的默认推断类型是: f64
