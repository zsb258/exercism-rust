#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };

    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            let mut hm = $crate::hashmap!();
            $(
                hm.insert($k, $v);
            )+
            hm
        }
    };
}
