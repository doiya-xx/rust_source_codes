fn main() {
    println!("Hello, world!");
    let _v: Vec<u32> = vec![1, 2, 3];
}

#[test]
fn m1() {
    let mut num = 5;

    {
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }
    {
        let r1 = &num;
        let r2 = &mut num;
    }

    {
        let address = 0x012345usize;
        let r = address as *const i32;
        // unsafe {
        //     println!("r is: {}", *r);
        // }
    }
}

#[test]
fn m2() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[test]
fn m3() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
