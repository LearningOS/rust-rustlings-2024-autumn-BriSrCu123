extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    #[no_mangle] // 确保该函数在链接时不会被修改名称
    pub extern "Rust" fn my_demo_function(a: u32) -> u32 { // 添加 extern "Rust"
        a
    }

    #[no_mangle] // 确保该函数在链接时不会被修改名称
    #[link_name = "my_demo_function"] // 指定链接名称
    pub extern "Rust" fn my_demo_function_alias(a: u32) -> u32 { // 添加 extern "Rust"
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // SAFETY: We know those functions are aliases of a safe Rust function.
        unsafe {
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456);
        }
    }
}
