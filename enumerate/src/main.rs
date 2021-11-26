#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    ip_kind: IpAddrKind,
    ip_address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(&four);
    route(&six);
    println!("four is {:?}, six is {:#?}", four, six);

    let home = IpAddr {
        ip_kind: IpAddrKind::V4,
        ip_address: String::from("127.0.0.1"),
    };
    println!("home is struct {:#?}", home);
    let lookback = IpAddr {
        ip_kind: IpAddrKind::V6,
        ip_address: String::from("::1"),
    };
    println!("lookback is struct {:#?}", lookback);
}

fn route(ip_kind: &IpAddrKind) {
    println!("ip_kind from func route is {:#?}", ip_kind);
}
