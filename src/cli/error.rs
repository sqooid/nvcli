pub enum Error {
    Argument {
        message: String,
        error: Box<dyn std::error::Error>,
    },
}

pub type Result<T> = std::result::Result<T, String>;
