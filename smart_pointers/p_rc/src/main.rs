// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
//
// fn m1() {
//     let a = Cons(
//         5, Box::new(Cons(
//             10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a)); // 这里会 move a
//     let c = Cons(4, Box::new(a)); // 使得这里 a 失效
// }

use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn m1() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn m2() {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    
    *(value.borrow_mut()) += 10;
    
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn m3() {
    // 循环链表
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
    
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    println!("a next item = {:?}", a.tail());
}

fn m4() {
    // 树形结构
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    
}

fn main() {
    m4();
}
