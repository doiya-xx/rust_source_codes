fn m1() {
    panic!("crash and burn");
}

fn m2() {
    let v = vec![1, 2, 3];
    v[99];
}

use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
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

// 传播错误
fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 传播错误的简写：? 运算符
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 链式方法调用来进一步缩短代码
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn m7(){
    let username = match read_username_from_file_1() {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };
    println!("{}",username);
    
}

fn m8(){
    let f = File::open("hello.txt")?;
}

// use std::error::Error;
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }

fn m9() {
    use std::net::IpAddr;
    
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

fn main() {
    m9();
}
