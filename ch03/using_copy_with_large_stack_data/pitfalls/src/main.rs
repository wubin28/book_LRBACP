#[derive(Clone, Copy, Debug)]
struct MyStruct {
    data: [u8; 1000000],
}

fn main() {
    let s1 = MyStruct { data: [0; 1000000] };
    let s2 = s1;
    println!("s1.data[0]: {} \ns2.data[0]: {}", s1.data[0], s2.data[0]);
}
// 运行输出：
// s1.data[0]: 0
// s2.data[0]: 0
