fn main() {
    // 创建一个数组
    let numbers = [1, 2, 3, 4, 5];

    // 创建一个切片，引用整个数组
    let slice_all: &[i32] = &numbers[..];
    println!("完整切片: {:?}", slice_all);

    // 创建一个切片，只引用数组的一部分
    let slice_partial: &[i32] = &numbers[1..4];
    println!("部分切片: {:?}", slice_partial);

    // 使用切片作为函数参数
    print_slice(slice_all);
    print_slice(slice_partial);

    // 从 Vec 创建切片
    let vec = vec![10, 20, 30, 40, 50];
    let vec_slice = &vec[2..];
    println!("从 Vec 创建的切片: {:?}", vec_slice);

    // 展示切片的动态大小特性
    print_slice(&numbers[..2]);
    print_slice(&numbers[..4]);

    // 使用切片进行安全访问
    for item in slice_partial {
        println!("切片中的项: {}", item);
    }
}

// 函数接受切片作为参数，可以处理不同大小的数组或向量
fn print_slice(slice: &[i32]) {
    println!("切片的长度: {}", slice.len());
    println!("切片的内容: {:?}", slice);
}
// 输出：
// 完整切片: [1, 2, 3, 4, 5]
// 部分切片: [2, 3, 4]
// 切片的长度: 5
// 切片的内容: [1, 2, 3, 4, 5]
// 切片的长度: 3
// 切片的内容: [2, 3, 4]
// 从 Vec 创建的切片: [30, 40, 50]
// 切片的长度: 2
// 切片的内容: [1, 2]
// 切片的长度: 4
// 切片的内容: [1, 2, 3, 4]
// 切片中的项: 2
// 切片中的项: 3
// 切片中的项: 4
