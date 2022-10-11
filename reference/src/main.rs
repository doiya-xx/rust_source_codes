fn main(){
    main1();
    main2();
    
    let mut s = String::from("hello");
    
    // 在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败：
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    
    // 不能在拥有不可变引用的同时拥有可变引用
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题
    println!("{}, {}, and {}", r1, r2, r3);
    
    // 将引用看成内存管理，不可变引用是读，可变引用是读写，避免出现数据竞争问题，则不能出现同时读写。
    
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

fn main1() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生

fn main2() {
    let mut s = String::from("hello");
    
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

