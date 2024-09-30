fn main() {
    // 栈内存使用
    // 固定大小，存储在栈上
    let stack_array: [i32; 5] = [1, 2, 3, 4, 5];

    // 堆内存使用
    // 动态大小，存储在堆上
    let heap_array: Vec<i32> = vec![1, 2, 3, 4, 5];
    // 大型固定大小数组，存储在堆上
    let large_array: Box<[i32; 1000]> = Box::new([0; 1000]);

    println!("栈数组: {:?}", stack_array);
    println!("堆Vec: {:?}", heap_array);
    println!("堆大数组长度: {}", large_array.len());
}
// 输出：
// 栈数组: [1, 2, 3, 4, 5]
// 堆Vec: [1, 2, 3, 4, 5]
// 堆大数组长度: 1000