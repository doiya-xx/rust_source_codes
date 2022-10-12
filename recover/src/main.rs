fn m1() {
    panic!("crash and burn");
}

fn m2() {
    let v = vec![1, 2, 3];
    v[99];
}

use std::fs::File;
use std::io::ErrorKind;

fn m3() {
    // 打开文件，返回Result枚举
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file, // 打开成功返回文件句柄
        Err(error) => match error.kind() { // 打开失败，匹配失败类型
            // 没有找到文件。创建新文件。
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc, // 创建新文件成功，返回文件句柄
                Err(e) => panic!("Problem creating the file: {:?}", e), //创建文件失败，打印错误信息。
            },
            // 其他错误。打印错误信息。
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
fn m4() {
    // m3的另一种写法
    // 使用闭包的方法
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn m5() {
    // 代替match。Ok则返回Ok中的值，Err则调用panic!
    let f = File::open("hello.txt").unwrap();
}

fn m6() {
    // 和unwrap使用方式一样，只是会将错误信息传入Err
    let f = File::open("hello.txt").expect("Failed to op Hello.txt");
}

fn m7(){
    let f = File::open("hello.txt").expect("Failed to op Hello.txt");
}

fn m8(){
    let f = File::open("hello.txt").expect("Failed to op Hello.txt");
}

fn main() {
    m6();
}
