use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("a指向的节点 = {:?}", a.tail());

        // 创建`b`到`a`的引用
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
        println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
        println!("b指向的节点 = {:?}", b.tail());

        // 利用RefCell的可变性，创建了`a`到`b`的引用
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
        println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

        // 下面一行println!将导致循环引用
        // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
        // println!("a next item = {:?}", a.tail());
    }

    #[test]
    fn test_thread(){
        let handle =  thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
}