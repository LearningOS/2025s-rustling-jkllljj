// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("my_option is None");
    }

    // 修复2: 添加缺失的逗号
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修复3: 正确清空Vec
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 修复4: 正确交换变量值
    let mut value_a = 45;
    let mut value_b = 66;
    // 正确交换方式
    std::mem::swap(&mut value_a, &mut value_b);

    println!("value a: {}; value b: {}", value_a, value_b);
}
