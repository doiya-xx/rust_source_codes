fn main() {
    println!("Hello, world!");

    another_function();

    another_function_1(5);

    print_labeled_measurement(5, 'h');

    // 语句没有返回值，表达式有返回值
    // 表达式的末尾没有分号，加上分号就转换为语句
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    //
    let x = five();
    println!("This value of x is: {}", x);

    //
    let x = plus_one(5);
    println!("This value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

// 函数参数必须指定类型
fn another_function_1(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
    return;
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
