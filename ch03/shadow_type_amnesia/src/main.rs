fn main() {
    let x = "55";
    println!("初始 x: {}", x);

    // 遮蔽 x，将其从 &str 转为 i32
    let x = x.parse::<i32>().unwrap();
    println!("遮蔽后的 x: {}", x);

    // 试图再次将 x 作为 &str 使用，导致编译错误
    // println!("再次使用 x: {}", x.len()); // 取消注释这行以查看错误：i32 没有 len 方法
}
// 输出：
// 初始 x: 55
// 遮蔽后的 x: 55
