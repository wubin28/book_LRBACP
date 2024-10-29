// 1. 方案一：延长智能指针的生命周期
fn solution1() {
    // 将智能指针移到外层作用域，确保其生命周期覆盖使用期间
    let smart_ptr = Box::new(55);
    let raw_ptr = &*smart_ptr as *const i32;

    println!("智能指针管理的值: {}", smart_ptr);
    unsafe {
        println!("裸指针指向的值: {}", *raw_ptr);
    }

    // smart_ptr仍然有效，raw_ptr也依然有效
    unsafe {
        println!("裸指针依然可以安全访问: {}", *raw_ptr);
    }
} // smart_ptr在这里才被释放

// 2. 方案二：使用引用而不是裸指针
fn solution2() {
    let value_ref;

    {
        let smart_ptr = Box::new(55);
        // 这行会产生编译错误，因为引用的生命周期不够长
        // value_ref = &*smart_ptr; // 编译器会捕获这个错误
    }

    // 正确的做法是把引用和数据放在同一作用域
    let smart_ptr = Box::new(55);
    value_ref = &*smart_ptr;

    println!("通过引用访问值: {}", value_ref);
}

// 3. 方案三：使用 Rc (引用计数)智能指针
use std::rc::Rc;

fn solution3() {
    let raw_ptr;
    let smart_ptr;

    {
        // 使用Rc创建一个引用计数的智能指针
        let rc_ptr = Rc::new(55);
        // 克隆Rc，增加引用计数
        smart_ptr = Rc::clone(&rc_ptr);
        raw_ptr = &*rc_ptr as *const i32;

        println!("原始Rc指针的值: {}", rc_ptr);
    } // rc_ptr被释放，但数据依然存在因为smart_ptr仍然持有引用

    println!("smart_ptr仍然有效: {}", smart_ptr);
    unsafe {
        println!("裸指针仍然可以安全访问: {}", *raw_ptr);
    }
}

// 4. 方案四：使用Arc用于线程安全场景
use std::sync::Arc;

fn solution4() {
    let raw_ptr;
    let smart_ptr;

    {
        // 使用Arc创建一个原子引用计数的智能指针，适用于多线程场景
        let arc_ptr = Arc::new(55);
        // 克隆Arc，增加引用计数
        smart_ptr = Arc::clone(&arc_ptr);
        raw_ptr = &*arc_ptr as *const i32;

        println!("原始Arc指针的值: {}", arc_ptr);
    } // arc_ptr被释放，但数据依然存在因为smart_ptr仍然持有引用

    println!("smart_ptr仍然有效: {}", smart_ptr);
    unsafe {
        println!("裸指针仍然可以安全访问: {}", *raw_ptr);
    }
}

// 5. 方案五：使用内部可变性 RefCell
use std::cell::RefCell;

fn solution5() {
    let raw_ptr;
    let refcell = RefCell::new(Box::new(55));

    {
        let borrowed = refcell.borrow();
        raw_ptr = &**borrowed as *const i32;
        println!("RefCell中的值: {}", borrowed);
    } // borrowed被释放，但refcell依然持有数据

    // 可以继续使用refcell
    println!("RefCell仍然持有值: {}", refcell.borrow());
    unsafe {
        println!("裸指针仍然可以安全访问: {}", *raw_ptr);
    }
}

fn main() {
    println!("=== 方案一 ===");
    solution1();
    println!("\n=== 方案二 ===");
    solution2();
    println!("\n=== 方案三 ===");
    solution3();
    println!("\n=== 方案四 ===");
    solution4();
    println!("\n=== 方案五 ===");
    solution5();
}
