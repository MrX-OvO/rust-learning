struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct
struct RGB(u8, u8, u8);
struct Point3d(f32, f32, f32);

fn main() {
    let mut user1 = User {
        // 声明为mut，则struct中所以成员都mutable
        email: String::from("xyz@123.com"),
        username: String::from("MrX"),
        sign_in_count: 123,
        active: true,
    };
    println!(
        "user1\n username:{}, email:{}, sign_in_count:{}, active:{}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    user1.email = String::from("wxyz@123.com");
    println!(
        "user1\n username:{}, email:{}, sign_in_count:{}, active:{}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    let user2 = build_user(String::from("Xiao Ming"), String::from("XiaoMing@123.com"));
    println!(
        "user2\n username:{}, email:{}, sign_in_count:{}, active:{}",
        user2.username, user2.email, user2.sign_in_count, user2.active
    );

    let user3 = User {
        email: String::from("User3@123.com"),
        username: String::from("User3"),
        //sign_in_count: user2.sign_in_count,
        //active: user2.active,
        ..user2 // struct更新语法
    };
    println!(
        "user3\n username:{}, email:{}, sign_in_count:{}, active:{}",
        user3.username, user3.email, user3.sign_in_count, user3.active
    );

    let red = RGB(255, 0, 0);
    println!(
        "rgb value of red color is R:{}, G:{}, B:{}",
        red.0, red.1, red.2
    );

    let point_of_origin = Point3d(0.0, 0.0, 0.0);
    println!(
        "point of origin is ({}, {}, {})",
        point_of_origin.0, point_of_origin.1, point_of_origin.2
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        //username: username, // 同名可简写
        username,
        //email: email,
        email,
        sign_in_count: 0,
        active: false,
    }
}
