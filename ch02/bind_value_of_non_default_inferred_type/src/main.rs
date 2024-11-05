fn main() {
    // 在变量声明时直接标注类型
    let integer: u8 = 55;
    println!("55的类型是: {}", std::any::type_name_of_val(&integer));

    // 在数值字面量后添加类型后缀
    let float = 3.14f32;
    println!("3.14的类型是: {}", std::any::type_name_of_val(&float));
}
// 运行输出：
// 55的类型是: u8
// 3.14的类型是: f32
