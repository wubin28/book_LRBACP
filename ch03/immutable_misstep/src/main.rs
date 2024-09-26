fn main() {
    // 错误示例
    let sum = 0;
    for i in 1..=3 {
        // sum += i;  // 取消注释这行以查看编译错误
        println!("当前循环值: {}", i);
    }
    println!("求和结果（实际上没有计算）: {}\n", sum);

    // 修复示例
    let mut sum = 0;
    for i in 1..=3 {
        sum += i;
        println!("当前循环值: {}, 当前和: {}", i, sum);
    }
    println!("正确的求和结果: {}\n", sum);

    // 使用函数式编程方法
    let sum: i32 = (1..=3).sum();
    println!("使用函数式方法的求和结果: {}", sum);
}
// 输出:
// 当前循环值: 1
// 当前循环值: 2
// 当前循环值: 3
// 求和结果（实际上没有计算）: 0
// 
// 当前循环值: 1, 当前和: 1
// 当前循环值: 2, 当前和: 3
// 当前循环值: 3, 当前和: 6
// 正确的求和结果: 6
// 
// 使用函数式方法的求和结果: 6