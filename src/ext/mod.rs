use anyhow::Result;
use sql_builder::SqlBuilder;

pub trait SqlBuilderExt {
    /// The same as .sql(), but does log::debug before returning the string
    fn sqld(&self) -> Result<String>;
}

impl SqlBuilderExt for SqlBuilder {
    fn sqld(&self) -> Result<String> {
        let sql = self.sql()?;
        log::debug!("{}", sql);
        Ok(sql)
    }
}

pub trait ExpectExt<T> {
    fn expect_or_exit(self, message: &str) -> T;
}

impl<T> ExpectExt<T> for Option<T> {
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

impl<T, E:std::fmt::Display> ExpectExt<T> for Result<T, E> {
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
