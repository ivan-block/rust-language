#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
};


fn main() {

    let home = IpAddress::V4("127.0.0.1");
    let data = IpAddress::V6("1");
    println!("{:?}", data);
}
