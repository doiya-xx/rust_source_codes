use std::ops::Deref;
use crate::List::{Cons, Nil};

fn m1() {
    // 5被分配在堆上，返回一个指向5的地址给在栈上的智能指针
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn m2() {
    let _list = Cons(
        1, Box::new(Cons(
            2, Box::new(Cons(
                3, Box::new(Nil))))));
}

fn m3() {
    let x = 5;
    let y = &x;
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 自定义智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn m4() {
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // 不懂如何解引用，没有实现解引用* trait
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}

fn m5() {
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // 这里就可以通过编译了
    // *y <==> *(y.deref()) <==> *(&T) <==> T
    
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn m6() {
    let m = MyBox::new(String::from("Rust"));
    // 这里 & 会调用deref()将&MyBox<Sting>转化为&Sting，
    // 然后编译器发现形参是&str，会继续强制转换，使用Sting的deref()，转换为&str
    hello(&m);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn m7() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn main() {
    m6();
}