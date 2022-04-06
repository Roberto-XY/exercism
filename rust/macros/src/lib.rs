#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr,)+) => { crate::hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}
