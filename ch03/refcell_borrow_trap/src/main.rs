use std::cell::RefCell;

fn main() {
    let point = RefCell::new((0, 0));

    let borrowed = point.borrow(); // 获取不可变借用

    // 尝试在已有不可变借用的情况下获取可变借用
    let mut_borrowed = point.borrow_mut(); // 这里会panic

    println!("x: {}, y: {}", borrowed.0, mut_borrowed.1); // 这行不会被执行
}
// 运行时panic:
// thread 'main' panicked at src/main.rs:9:30:
// already borrowed: BorrowMutError