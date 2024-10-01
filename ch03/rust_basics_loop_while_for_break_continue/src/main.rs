fn main() {
    // loop 循环示例
    println!("1. loop 循环示例:");
    let mut counter = 0;
    loop {
        println!("  循环计数: {}", counter);
        counter += 1;
        if counter >= 3 {
            break;
        }
    }

    // while 循环示例
    println!("\n2. while 循环示例:");
    let mut number = 0;
    while number < 3 {
        println!("  数字是: {}", number);
        number += 1;
    }

    // for 循环示例
    println!("\n3. for 循环示例:");
    for i in 0..3 {
        println!("  索引是: {}", i);
    }

    // break 和 continue 示例
    println!("\n4. break 和 continue 示例:");
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                println!("  继续外层循环");
                continue 'outer;
            }
            if i == 2 && j == 2 {
                println!("  跳出所有循环");
                break 'outer;
            }
            println!("  i = {}, j = {}", i, j);
        }
    }
}
// 输出：
// 1. loop 循环示例:
//   循环计数: 0
//   循环计数: 1
//   循环计数: 2
//
// 2. while 循环示例:
//   数字是: 0
//   数字是: 1
//   数字是: 2
//
// 3. for 循环示例:
//   索引是: 0
//   索引是: 1
//   索引是: 2
//
// 4. break 和 continue 示例:
//   i = 0, j = 0
//   i = 0, j = 1
//   i = 0, j = 2
//   i = 1, j = 0
//   继续外层循环
//   i = 2, j = 0
//   i = 2, j = 1
//   跳出所有循环
