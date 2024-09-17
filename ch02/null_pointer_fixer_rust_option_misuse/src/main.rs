fn get_name(id: i32) -> Option<String> {
    // 模拟数据库查询
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn print_name(id: i32) {
    // 直接使用 get_name(id) 的结果,而不进行匹配
    println!("Name: {}", get_name(id));
}

fn main() {
    print_name(1);
    print_name(2);
}
// // Output of 'cargo build'
// error[E0277]: `Option<String>` doesn't implement `std::fmt::Display`
//   --> src/main.rs:12:26
//    |
// 12 |     println!("Name: {}", get_name(id));
//    |                          ^^^^^^^^^^^^ `Option<String>` cannot be formatted with the default formatter
//    |
//    = help: the trait `std::fmt::Display` is not implemented for `Option<String>`
//    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `null_pointer_fixer_rust_option_misuse` (bin "null_pointer_fixer_rust_option_misuse") due to 1 previous error
