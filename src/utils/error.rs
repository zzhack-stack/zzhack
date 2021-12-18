use crate::console_log;
use std::fmt::Display;

pub trait ErrorDebug<T, E> {
    fn debug(&self) -> &T;
}

impl<T, E> ErrorDebug<T, E> for Result<T, E>
where
    E: Display,
{
    fn debug(&self) -> &T {
        match &self {
            Ok(data) => data,
            Err(err) => {
                console_log!("{}", err);
                panic!("{}", err)
            }
            _ => panic!("foo"),
        }
    }
}
