fn main() {
    let mut x: u8 = 255;
    println!("Initial value: {}", x);

    // 演示溢出问题
    x = x + 1; // 在debug模式下会panic,在release模式下会wrap
    println!("After adding 1 (overflow): {}", x);

    // 使用wrapping_add方法处理溢出
    x = 255;
    x = x.wrapping_add(1);
    println!("After wrapping_add 1: {}", x);

    x = x.wrapping_add(1);
    println!("After wrapping_add 1 again: {}", x);
}
// 运行’cargo run'的debug模式的输出：
// Initial value: 255
// thread 'main' panicked at src/main.rs:6:9:
// attempt to add with overflow
//
// 运行’cargo build --release'后运行target/release目录下的可执行文件的输出：
// Initial value: 255
// After adding 1 (overflow): 0
// After wrapping_add 1: 0
// After wrapping_add 1 again: 1