fn main() {
    let result = find_first_even(&[1, 3, 5, 6, 7, 8]);
    println!("找到的第一个偶数: {:?}", result);
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    let mut iter = numbers.iter();
    loop {
        match iter.next() {
            Some(&num) if num % 2 == 0 => break Some(num),
            Some(_) => continue,
            None => break None,
        }
    }
}
// 输出：
// 找到的第一个偶数: Some(6)
