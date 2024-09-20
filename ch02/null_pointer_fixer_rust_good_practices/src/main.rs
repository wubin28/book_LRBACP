fn get_name(id: i32) -> Option<String> {
    // 模拟数据库查询
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn print_name_by_match(id: i32) {
    match get_name(id) {
        Some(name) => println!("Name: {}", name),
        None => println!("No name found for id: {}", id),
    }
}

fn print_name_by_if_let(id: i32) {
    if let Some(name) = get_name(id) {
        println!("Name: {}", name);
    } else {
        println!("No name found for id: {}", id);
    }
}

fn print_name_by_unwrap_or(id: i32) {
    let name_unknown = get_name(id).unwrap_or(String::from("Unknown"));
    let name_for_id = get_name(id).unwrap_or_else(|| format!("No name found for id: {}", id));
    println!("Name: {}", name_unknown);
    println!("Name: {}", name_for_id);
}

fn print_name_by_unwrap_not_recommended(id: i32) {
    println!("Name: {}", get_name(id).unwrap());
}

fn main() {
    print_name_by_match(1); // 这会打印名字
    print_name_by_match(2); // 这会打印未找到名字的消息

    print_name_by_if_let(1); // 这会打印名字
    print_name_by_if_let(2); // 这会打印未找到名字的消息

    print_name_by_unwrap_or(1); // 这会打印名字
    print_name_by_unwrap_or(2); // 这会打印未找到名字的消息

    print_name_by_unwrap_not_recommended(1); // 这会打印名字
    print_name_by_unwrap_not_recommended(2); // 这会打印未找到名字的消息
}
// Output:
// Name: Alice
// No name found for id: 2
// Name: Alice
// No name found for id: 2
// Name: Alice
// Name: No name found for id: 2
// Name: Alice
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', main.rs:31:39
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
