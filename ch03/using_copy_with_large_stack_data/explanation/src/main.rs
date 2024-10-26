// MyStruct结构体里有大型数组，不适合实现Copy trait
// Copy trait仅用于简单和轻量的按位复制，而不适用于大型数据结构
// 正确写法应该是：#[derive(Clone, Debug)]
#[derive(Clone, Copy, Debug)]
struct MyStruct {
    // 大型数组
    data: [u8; 1000000],
}

fn main() {
    let s1 = MyStruct { data: [0; 1000000] };
    // 正确写法应该是：let s2 = s1.clone();
    // 调用clone()方法的好处是显式地提醒程序员这里要进行昂贵的克隆操作
    let s2 = s1;
    println!("s1.data[0]: {} \ns2.data[0]: {}", s1.data[0], s2.data[0]);
}
