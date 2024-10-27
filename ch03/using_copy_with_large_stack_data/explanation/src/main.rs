// MyStruct结构体里有大型数组，不适合实现Copy trait
// Copy trait仅用于简单和轻量的按位复制，而不适用于大型数据结构
// 正确写法应该是：#[derive(Clone)]
#[derive(Clone, Copy)]
struct MyStruct {
    // 大型数组
    data: [u8; 1000000],
}

fn main() {
    let s1 = MyStruct { data: [0; 1000000] };
    // 好一些的写法应该是：let s2 = s1.clone();
    // 调用clone()方法的好处是显式地提醒程序员这里要进行昂贵的克隆操作
    // 但这样改后，clone()操作仍然发生在栈上，因为数组[u8; 1000000]是固定大小的类型
    // 在Rust中，固定大小的数组是存储在栈上的
    // MyStruct结构体包含的是整个数组，而不是数组的引用
    // 所以整个MyStruct都在栈上分配
    // clone()会在栈上创建完整的数据副本
    // Rust默认的栈大小通常是几MB
    // MyStruct本身就占用约1MB栈空间
    // 栈溢出风险仍然存在
    // 要真正避免栈溢出，需要修改数据结构设计
    let s2 = s1;
    println!("s1.data[0]: {} \ns2.data[0]: {}", s1.data[0], s2.data[0]);
}
