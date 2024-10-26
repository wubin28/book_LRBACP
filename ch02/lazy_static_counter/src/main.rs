use lazy_static::lazy_static;

static COUNTER: i32 = 0;

lazy_static! {
    static ref REFERENCE: &'static i32 = &COUNTER;
}

fn main() {
    println!("Reference value: {}", *REFERENCE);
}
// 输出：
// Reference value: 0
