#[macro_export]
macro_rules! function_path {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            ::std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let mut name = &name[..(name.len() - 3)];
        loop {
            let mut stripped = false;
            for suffix in [
                "::{{closure}}",
                "::test_impl", // fn wrapped by #[test_log::test]
            ] {
                if let Some(stripped_name) = name.strip_suffix(suffix) {
                    stripped = true;
                    name = stripped_name;
                }
            }
            if !stripped {
                break;
            }
        }
        name
    }};
}
