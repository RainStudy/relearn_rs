use std::{error::Error, fmt::{Display, self}};

pub type Resultish<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct AppError {
    message: String
}

impl AppError {
    pub fn new(message: String) -> Box<Self> {
        Box::new(Self { message })
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "异常信息: {}", self.message)
    }
}

impl Error for AppError {}