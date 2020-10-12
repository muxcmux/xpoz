pub trait ExpectOrExit<T> {
    fn expect_or_exit(self, message: &str) -> T;
}

impl<T> ExpectOrExit<T> for Option<T> {
    fn expect_or_exit(self, message: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                log::error!("{}", message);
                std::process::exit(1);
            }
        }
    }
}

impl<T, E:std::fmt::Display> ExpectOrExit<T> for Result<T, E> {
    fn expect_or_exit(self, message: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}: {}", message, e);
                std::process::exit(1);
            }
        }
    }
}