fn main() {
    let value = 55;
    println!("初始 value: {}", value);

    let value = value.to_string();
    println!("遮蔽后的 value: {}", value);

    let value = value.len();
    println!("再次遮蔽后的 value 长度: {}", value);
}
// 输出：
// 初始 value: 55
// 遮蔽后的 value: 55
// 再次遮蔽后的 value 长度: 2
