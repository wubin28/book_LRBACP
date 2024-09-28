fn main() {
    // 问题代码
    let x: i32;
    if true {
        x = 5;
    }
    // 取消注释下面一行以查看编译错误: 变量`x`在某些执行路径上未初始化
    // println!("x的值是: {}", x);

    // 修复代码
    let y: i32;
    if true {
        y = 5;
    } else {
        y = 10;
    }
    println!("y的值是: {}", y); // 输出: y的值是: 5
}
// 输出:
// y的值是: 5
