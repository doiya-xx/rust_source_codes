fn main1() {
    // vector: 相同类型的值在堆上相邻排列
    let _v: Vec<i32> = Vec::new();
    // 使用vec!宏
    {
        let _v = vec![1, 2, 3];
        // 处理变量 v
    } // <- 这里 v 离开作用域并被丢弃
    
    // 更新vector
    {
        let mut v: Vec<i32> = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }
    
    // 读取vector
    {
        let v = vec![1, 2, 3, 4, 5];
        
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }
    
    // 遍历vector
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", *i);
        }
        println!("{:?}", &v);
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        println!("{:?}", &v);
    }
    
    //
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        
        let row: Vec<SpreadsheetCell> = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}

fn main2() {
    //String
    // 创建字符串
    {
        let data = "initial contents";
        let _s = data.to_string();
        // 该方法也可直接用于字符串字面量：
        let _s = "initial contents".to_string();
        
        let mut _s = String::new();
        let _s = String::from("initial contents");
    }
    // 更新字符串
    {
        // 插入字符串
        let mut s = String::from("foo");
        s.push_str("bar");
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);
        // 插入字符
        let mut s = String::from("lo");
        s.push('l');
        // 使用运算符+或format!宏
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let _s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
        //
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let _s = s1 + "-" + &s2 + "-" + &s3;
        //<==>
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let _s = format!("{}-{}-{}", s1, s2, s3); // 不会获得仍和参数的所有权
    }
    // 索引字符串
    {
        // 不能使用索引来引用单独字符
        // let s1 = String::from("hello");
        // let h = s1[0];
        // String使用Vec<u8>封装，使用UTF-8进行编码，就会出现不用语言使用不同字节数进行编码
        // 会导致看起来占四字节的字符串，实际需要更多字节数进行编码。
        // 进而使用索引不会返回期望的字符。
        let hello = "Здравствуйте";
        
        let s = &hello[0..4]; // “Зд”
    }
    // 遍历字符串
    {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }
        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}

fn main3() {
    use std::collections::HashMap;
    
    // hash map
    //
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    
        let teams  = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // 这里 field_name 和 field_value 不再有效，
        // 尝试使用它们看看会出现什么编译错误！
    }
    // 访问hash map
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
    
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        for (key, value) in &scores {
            println!("{}: {}", *key, *value);
        }
    }
    // update hash map
    {
        // 插入原有的键，值会被替换
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        println!("{:?}", scores);
        // 只在键没有对应值时插入
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
        // 根据旧值更新一个值
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
        
    }
}

fn main() {
    main1();
    main2();
    main3();
}
