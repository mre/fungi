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

pub fn try() {
    a::series::of::nested_modules();
    of::nested_modules();
}
