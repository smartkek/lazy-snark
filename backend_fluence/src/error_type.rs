use std::error::Error;

pub type AppResult<T> = ::std::result::Result<T, Box<Error>>;
