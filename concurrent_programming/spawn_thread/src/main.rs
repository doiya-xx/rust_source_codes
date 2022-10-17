use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn _m1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // 如果没有调用线程的join()方法，则会在主线程结束后结束
    // join()会阻塞当前线程，直到调用join()的线程结束
    handle.join().unwrap();
}

fn _m2() {
    let v = vec![1, 2, 3];
    
    // 在线程中借用主线程的对象
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    
    handle.join().unwrap();
}

// 使用消息传递在线程间传送数据
// 通道 channel
// mpsc: multiple producer, single consumer
fn _m3() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        // 在线程中发送数据，同时发送所有权
        tx.send(val).unwrap();
        // println!("val is  {}", val);
    });
    
    // 在主线程中接受数据
    // recv() 会阻塞当前线程，直到在通道中接受到数据
    // try_recv() 不会阻塞线程，会立刻返回 Result<T, E>
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn _m4() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}

fn m5() {
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}

fn main(){
    m5();
}
