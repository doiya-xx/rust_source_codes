pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// 模块
// Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的
fn serve_order() {}

mod front_of_house;

// use front_of_house::hosting;
pub use crate::front_of_house::hosting;

mod back_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    // front_of_house::hosting::add_to_waitlist();
    //---
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    //---
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    //---
    // use关键词
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。
use std::collections::HashMap;
use std::fmt;
use std::io;
// 给引入起别名
use std::fmt::Result;
use std::io::Result as IoResult;
//---
// use std::cmp::Ordering;
// use std::io;
// <==>
// use std::{cmp::Ordering, io};
//---
// use std::io;
// use std::io::Write;
// <==>
// use std::io::{self, Write};

// use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// fn function1() -> fmt::Result {
//     // --snip--
//     fmt::Result
// }
//
// fn function2() -> io::Result<()> {
//     // --snip--
//     io::Result
// }

