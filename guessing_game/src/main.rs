// 将库引入到当前作用域中
// 标志库std::
// 输入/输出库io
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// 程序的入口点
// fn声明一个新函数，参数域()，函数体{}
fn main() {
    // println!: 是一个在屏幕上打印字符串的宏
    println!("Guess the number!");

    // 获取一个随机数生成器的实例，调用生成器生成随机数函数。
    // 1..101表示范围1-100，等价于1..=100
    let secret_number = rand::thread_rng()
        .gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // let声明用来创建变量，变量默认不可变；mut使一个变量可变
        // :: 表明new()是String类型的一个关联函数
        let mut guess = String::new();

        // stdin()返回一个终端标准输入句柄类型的实例
        io::stdin()
            .read_line(&mut guess) // 从标准输入句柄获取用户输入，赋值给&mut guess
            .expect("Failed to read line");

        let guess: u32 = match guess.trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {}表示占位符
        // println!("You guessed: {}", guess);

        // cmp()返回Ordering枚举类型
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
