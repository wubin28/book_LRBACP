// 为MyStruct结构体添加了三个派生的trait：
// Clone：允许通过显式调用.clone()方法来复制结构体。
// Copy：允许结构体在赋值或传参时执行浅拷贝，而不移动所有权。
#[derive(Clone, Copy)]
// 定义了一个名为MyStruct的结构体
struct MyStruct {
    // 字段data的类型为u8的数组，大小为100万个元素
    data: [u8; 1000000],
}

// main函数是程序的入口点
fn main() {
    // 将MyStruct结构体实例绑定到变量s1上，其中data字段每个元素都被设置为0
    let s1 = MyStruct { data: [0; 1000000] };
    // 因为MyStruct派生了Copy trait，所以赋值时用浅拷贝的方式将s1的数据复制并绑定到s2上
    let s2 = s1;
    // s1依然有效，因为没有发生所有权转移
    println!("s1.data[0]: {} \ns2.data[0]: {}", s1.data[0], s2.data[0]);
}
// 运行输出：
// s1.data[0]: 0
// s2.data[0]: 0
