enum IpAddrKind {
    V4,
    V6,
}

// using structs
fn one() {
    struct IpAddr {
        #[allow(dead_code)] kind: IpAddrKind,
        #[allow(dead_code)] address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

// using enums
fn two() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));
}

// using enums and different types of associated data
fn three() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home = IpAddr::V4(127, 0, 0, 1);

    let _loopback = IpAddr::V6(String::from("::1"));
}

fn route(_ip_type: IpAddrKind) {}

pub fn sample() {
    one();
    two();
}
