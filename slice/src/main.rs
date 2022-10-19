fn main() {
    let mut s = String::from("hello world");
    
    let word = first_word(&s); // word 的值为 5
    
    s.clear(); // 这清空了字符串，使其等于 ""
    
    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
    
    // slice
    let hello: &str = &s[0..5]; // &s[..5]
    let world = &s[6..11]; // &s[5..]
    
    // 数组slice
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{:?}",bytes);
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main3() {
    let my_string = String::from("hello world");
    
    // `first_word` 接受 `String` 的切片，无论是部分还是全部
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // `first_word` 也接受 `String` 的引用，
    // 这等同于 `String` 的全部切片
    let word = first_word3(&my_string);
    
    let my_string_literal = "hello world";
    
    // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);
    
    // 因为字符串字面值**就是**字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word3(my_string_literal);
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}