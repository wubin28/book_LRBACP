fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    println!("数组: {:?}", numbers);
    println!("使用迭代器访问元素:");

    for (index, &value) in numbers.iter().enumerate() {
        println!("索引 {}: 值 {}", index, value);
    }

    println!("\n筛选偶数:");
    for (index, &value) in numbers.iter().enumerate().filter(|(_, &x)| x % 2 == 0) {
        println!("索引 {}: 值 {}", index, value);
    }
}
// 输出：
// 数组: [1, 2, 3, 4, 5]
// 使用迭代器访问元素:
// 索引 0: 值 1
// 索引 1: 值 2
// 索引 2: 值 3
// 索引 3: 值 4
// 索引 4: 值 5
//
// 筛选偶数:
// 索引 1: 值 2
// 索引 3: 值 4
