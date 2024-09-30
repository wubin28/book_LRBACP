fn main() {
    let result = find_first_even(&[1, 3, 5, 6, 7, 8]);
    println!("找到的第一个偶数: {:?}", result);
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}
// 输出：
// 找到的第一个偶数: Some(6)
