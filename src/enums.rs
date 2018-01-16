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

// https://doc.rust-lang.org/stable/std/net/enum.IpAddr.html
fn four() {
    #[allow(dead_code)]
    struct Ipv4Addr {
        // details elided
    }

    #[allow(dead_code)]
    struct Ipv6Addr {
        // details elided
    }

    #[allow(dead_code)]
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}

fn five() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // struct QuitMessage; // unit struct
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // tuple struct
    // struct ChangeColorMessage(i32, i32, i32); // tuple struct

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

pub fn sample() {
    one();
    two();
    three();
    four();
    five();
}
