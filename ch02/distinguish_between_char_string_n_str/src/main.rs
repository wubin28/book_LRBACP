fn main() {
    let c = '中';
    println!("c: {}", c);
    println!("c的类型是: {}", std::any::type_name_of_val(&c));
    println!("c所占用的字节数是: {}", std::mem::size_of_val(&c));

    // 下面这行也可以这样写：let string = String::from("中");
    let string: String = c.to_string();
    println!("string: {}", string);
    println!("string的类型是: {}", std::any::type_name_of_val(&string));
    println!("string所占用的字节数是: {}", std::mem::size_of_val(&string));

    // 下面这行也可以这样写：let s: &str = &string;
    let s: &str = &c.to_string(); // 将 `char` 转为 `String`，再转换为 `&str`
    println!("s: {}", s);
    println!("s的类型是: {}", std::any::type_name_of_val(&s));
    println!("s所占用的字节数是: {}", std::mem::size_of_val(&s));
}
// 运行输出：
// c: 中
// c的类型是: char
// c所占用的字节数是: 4
// string: 中
// string的类型是: alloc::string::String
// string所占用的字节数是: 24
// s: 中
// s的类型是: &str
// s所占用的字节数是: 16
