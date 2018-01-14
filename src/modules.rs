pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("a::series::of::nested_modules")
            }
        }
    }
}

pub mod another {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("another::series::of::nested_modules")
            }
        }
    }
}

use self::another::series::of;

use std::fmt;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl fmt::Debug for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TrafficLight::*;
        match self {
            &Red => write!(f, "TrafficLight is RED"),
            &Yellow => write!(f, "TrafficLight is YELLOW"),
            &Green => write!(f, "TrafficLight is GREEN"),
        }
    }
}

use self::TrafficLight::{Red, Yellow};

pub fn try() {
    a::series::of::nested_modules();
    of::nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
    println!("{:?}/{:?}/{:?}", red, yellow, green);
}
