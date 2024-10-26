fn main() {
    let result = find_first_even(&[1, 3, 5, 6, 7, 8]);
    println!("找到的第一个偶数: {:?}", result);
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            break Some(num); // 错误：不能在 for 循环中使用带值的 break
        }
    }
    None
}
// 编译错误：
// error[E0571]: `break` with value from a `for` loop
//  --> src/main.rs:9:13
//   |
// 7 |     for &num in numbers {
//   |     ------------------- you can't `break` with a value in a `for` loop
// 8 |         if num % 2 == 0 {
// 9 |             break Some(num); // 错误：不能在 for 循环中使用带值的 break
//   |             ^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
//   |
// help: use `break` on its own without a value inside this `for` loop
//   |
// 9 |             break; // 错误：不能在 for 循环中使用带值的 break
//   |             ~~~~~