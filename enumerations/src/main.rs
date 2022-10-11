// enum IpAddrKind {
//     // variants
//     V4,
//     V6,
// }
//
// fn main1() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//
//     route1(IpAddrKind::V4);
//     route1(IpAddrKind::V6);
// }
//
// fn route1(ip_type: IpAddrKind) {}
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// fn main2() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }
//
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
//
// fn main3() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }
//
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
//
// fn main4() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
// }

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

struct QuitMessage;

// 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

// 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

fn main5() {
    let m = Message::Write(String::from("hello"));
    m.call();
    dbg!(&m);
    println!("{:?}", m);
}

fn main6() {
    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: Option<i32> = None;
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    let sum = x + y; // 类型不同无法相加
}

fn main() {
    // main1();
    main5();
}