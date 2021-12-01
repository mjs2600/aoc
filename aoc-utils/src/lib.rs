#[macro_export]
macro_rules! read_input {
    ($year:expr, $e:expr) => {{
        include_str!(concat!("../../input/", $year, "-", $e, ".txt"))
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
