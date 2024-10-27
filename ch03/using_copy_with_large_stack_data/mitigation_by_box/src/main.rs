#[derive(Clone)]
struct MyStruct {
    data: Box<[u8; 1000000]>, // 将固定大小数组放在堆上
}

fn main() {
    let s1 = MyStruct {
        data: Box::new([0; 1000000]), // 数组分配在堆上
    };
    let s2 = s1.clone(); // 在堆上创建新的数组副本
    println!("s1.data[0]: {} \ns2.data[0]: {}", s1.data[0], s2.data[0]);
}
// 运行输出：
// s1.data[0]: 0
// s2.data[0]: 0
