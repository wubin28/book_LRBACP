#[derive(Clone)]
struct MyStruct {
    data: Vec<u8>, // 数据存储在堆上，栈上只存储Vec的头部信息
}

fn main() {
    let s1 = MyStruct {
        data: vec![0; 1000000], // 数据分配在堆上
    };
    let s2 = s1.clone(); // 只在堆上创建新的数据副本，栈上开销很小
    println!("s1.data[0]: {} \ns2.data[0]: {}", s1.data[0], s2.data[0]);
}
// 运行输出：
// s1.data[0]: 0 
// s2.data[0]: 0
