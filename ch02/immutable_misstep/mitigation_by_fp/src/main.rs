fn main() {
    // let 用于声明一个变量 sum
    // : i32 显式指定变量类型为32位有符号整数
    // 下面是体现了函数式编程的链式调用：
    // (1..=3) 创建一个范围（Range）迭代器，包含1到3的所有整数（包括3）
    // ..= 是包含右边界的范围运算符，所以这里表示[1,2,3]
    // .sum() 是一个迭代器方法，
    // 对迭代器中的所有元素进行归约（reduction）操作，
    // 用于计算范围内所有数字的总和
    let sum: i32 = (1..=3).sum();
    println!("求和结果: {}\n", sum);
}
// 运行输出:
// 求和结果: 6
