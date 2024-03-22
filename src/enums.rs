/*
enum IpAddr {
    V4(String),
    V6(String),
}
*/

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn enums() {
    //https://curr.to/enum-u8-u8-u8-u8
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home {:?} Loopback {:?}", home, loopback);
}               