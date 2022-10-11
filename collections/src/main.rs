fn main1() {
    // vector: 相同类型的值在堆上相邻排列
    let v: Vec<i32> = Vec::new();
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
}

fn main() {
    main1();
}
