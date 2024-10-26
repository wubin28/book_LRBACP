fn main() {
    // 创建一个固定大小的数组，所有元素类型相同
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组 numbers: {:?}", numbers);

    // 使用 [T; N] 语法声明数组类型
    let fruits: [&str; 3] = ["苹果", "香蕉", "橙子"];
    println!("水果数组: {:?}", fruits);

    // 使用 [value; count] 语法初始化所有元素为相同值
    let zeros = [0; 5];
    println!("零数组: {:?}", zeros);

    // 访问数组元素
    println!("numbers 的第一个元素: {}", numbers[0]);
    println!("fruits 的最后一个元素: {}", fruits[fruits.len() - 1]);

    // 数组的大小是固定的，在编译时就确定
    println!(
        "numbers 数组的大小: {} 字节",
        std::mem::size_of_val(&numbers)
    );

    // 对比：使用 Vec 创建可变大小的集合
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("可变大小的 Vec: {:?}", vec);
}
// 输出：
// 数组 numbers: [1, 2, 3, 4, 5]
// 水果数组: ["苹果", "香蕉", "橙子"]
// 零数组: [0, 0, 0, 0, 0]
// numbers 的第一个元素: 1
// fruits 的最后一个元素: 橙子
// numbers 数组的大小: 20 字节
// 可变大小的 Vec: [1, 2, 3]
