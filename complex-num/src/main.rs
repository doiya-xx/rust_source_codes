use num::complex::Complex;

fn main() {
    // let a = Complex { re: 2.1, im: -1.2 };
    // let b = Complex::new(11.1, 22.2);
    // let result = a + b;
    //
    // println!("{}", result);
    
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束
    
    let r3 = &mut s;
    println!("{}", r3);
}

