fn main() {
    // 创建一个包含不同类型元素的元组
    let tuple = (55, "hello", 3.14);

    // 元组的大小是固定的,不能增加或减少元素
    println!("元组的大小: {} 字节", std::mem::size_of_val(&tuple));

    // 使用模式匹配(解构绑定)来获取元素
    let (x, y, z) = tuple;
    println!("解构: x = {}, y = {}, z = {}", x, y, z);

    // 使用下标访问元素
    println!(
        "下标访问: tuple.0 = {}, tuple.1 = {}, tuple.2 = {}",
        tuple.0, tuple.1, tuple.2
    );

    // 创建一个单元(unit)元组
    let unit = ();
    println!("单元类型的大小: {} 字节", std::mem::size_of_val(&unit));

    // 调用一个返回单元的函数
    print_unit();
}

// 这个函数不返回任何值,因此隐式返回单元值
fn print_unit() {
    println!("这个函数返回单元值");
}
// 输出：
// 元组的大小: 32 字节
// 解构: x = 55, y = hello, z = 3.14
// 下标访问: tuple.0 = 55, tuple.1 = hello, tuple.2 = 3.14
// 单元类型的大小: 0 字节
// 这个函数返回单元值
