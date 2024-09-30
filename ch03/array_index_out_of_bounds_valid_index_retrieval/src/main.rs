fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let indices = vec![0, 2, 4, 6]; // 注意：6是一个无效索引

    println!("数组: {:?}", numbers);
    println!("尝试访问的索引: {:?}", indices);

    for &index in &indices {
        if index < numbers.len() {
            println!("索引 {} 有效，值为: {}", index, numbers[index]);
        } else {
            println!("索引 {} 无效，跳过访问", index);
        }
    }
}
// 输出：
// 数组: [1, 2, 3, 4, 5]
// 尝试访问的索引: [0, 2, 4, 6]
// 索引 0 有效，值为: 1
// 索引 2 有效，值为: 3
// 索引 4 有效，值为: 5
// 索引 6 无效，跳过访问
