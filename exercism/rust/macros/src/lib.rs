// use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    ($( $k:expr => $v:expr),* $(,)*) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($k, $v);
            )*
            hm
        }
    };
}
