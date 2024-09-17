fn get_name(id: i32) -> Option<String> {
    // 模拟数据库查询
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn print_name(id: i32) {
    match get_name(id) {
        Some(name) => println!("Name: {}", name),
        None => println!("No name found for id: {}", id),
    }
}

fn main() {
    print_name(1); // 这会打印名字
    print_name(2); // 这会打印未找到名字的消息
}
// Output:
// Name: Alice
// No name found for id: 2