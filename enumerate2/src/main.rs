#[derive(Debug)]
// 枚举变型中可以嵌入任意数据类型
/*
struct Ipv4Addr {
    // --snip--
}
struct Ipv6Addr {
    // --snip--
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
*/
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// method
impl IpAddrKind {
    fn call(&self) {}
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    println!("home is {:#?}", home);
    home.call();
    let lookback = IpAddrKind::V6(String::from("::1"));
    println!("lookback is {:#?}", lookback);
}
