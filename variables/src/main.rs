fn main() {
    // 变量：可变和不可变
    /*
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // Cannot assign twice to immutable variable [E0384]
    println!("The value of x is: {}", x);
     */

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // Cannot assign twice to immutable variable [E0384]
    println!("The value of x is: {}", x);

    // 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 遮蔽
    let spaces = "   ";
    let spaces = spaces.len();

    /*
    let mut spaces = "   ";
    spaces = spaces.len(); // mismatched types [E0308] expected `&str`, found `usize`
     */

    let mut spaces = " ";
    let mut spaces = 123;
    let spaces = "   ";
    let spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");

    // 整数类型
    let a: u8 = 255;
    println!("{}", a);

    // 浮点数
    let x = 2.0;
    let y: f32 = 3.0;

    // 加减乘除取模
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    let floored = 2.0 / 3.0; // Results in 0.66666666666
    println!("{}", floored);

    let remainder = (-6) % 5;
    println!("{}", remainder);

    // 布尔类型
    let t = true;

    let f: bool = false; // with explicit type annotation

    // 字符类型
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0); // 索引
    let (x, y, z) = tup; // 解构
    println!("The value of y is: {}", y);

    // 数组类型
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a = [3;5]; // [3,3,3,3,3]

    let first = a[0];
    let last = a[1];

    // 函数
    another_function();

}

fn another_function(){
    println!("Another function.");
}