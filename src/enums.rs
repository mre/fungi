enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrNaive {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddrNaive {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddrNaive {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

fn sample() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
fn route(ip_type: IpAddrKind) { }
