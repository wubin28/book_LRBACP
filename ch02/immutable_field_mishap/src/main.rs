struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 错误示例
    let point = Point { x: 0, y: 0 };
    println!("初始点: ({}, {})", point.x, point.y);

    // point.x = 5;  // 取消注释这行以查看编译错误

    println!(
        "尝试修改点的 x 坐标（实际上没有修改）: ({}, {})\n",
        point.x, point.y
    );

    // 修复示例
    let mut point = Point { x: 0, y: 0 };
    println!("初始点: ({}, {})", point.x, point.y);

    point.x = 5;
    println!("成功修改点的 x 坐标: ({}, {})\n", point.x, point.y);

    // 使用内部可变性
    use std::cell::RefCell;

    // 创建一个不可变的Point实例
    let point_immutable = Point { x: 0, y: 0 };

    // 将不可变实例包装在RefCell中，实现内部可变性
    let point = RefCell::new(point_immutable);
    println!("初始点: ({}, {})", point.borrow().x, point.borrow().y);

    // 现在可以在不改变point变量本身可变性的情况下修改其内部值
    point.borrow_mut().x = 5;
    println!(
        "使用 RefCell 修改点的 x 坐标: ({}, {})",
        point.borrow().x,
        point.borrow().y
    );
}
// 输出:
// 初始点: (0, 0)
// 尝试修改点的 x 坐标（实际上没有修改）: (0, 0)
//
// 初始点: (0, 0)
// 成功修改点的 x 坐标: (5, 0)
//
// 初始点: (0, 0)
// 使用 RefCell 修改点的 x 坐标: (5, 0)
