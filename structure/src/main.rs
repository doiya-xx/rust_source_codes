// 结构体
struct User {
    // struct name
    // field
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    main1();
    main2();
}

fn main1() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    
    // 不使用更新语法
    // 这里将user1的类型为String的字段复制给user2
    // 会使user1将数据移动到user2，使得user1被移动；
    // 想要user1不被移动，就得使用clone
    let user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // 使用更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 元组结构体（tuple struct）
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);
// 类单元结构体（unit-like structs）
struct AlwaysEqual;

fn main2() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    let subject = AlwaysEqual;
}
