use std::fmt;
use std::io;

pub fn read_first_arg() -> Result<String, MyError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Err(MyError::InputNotProvided);
    }
    Ok(args[0].to_owned())
}

#[derive(Debug)]
pub enum MyError {
    InputNotProvided,
    FileNotFound(io::Error),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> MyError {
        MyError::FileNotFound(error)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::InputNotProvided => write!(f, "Input file must be provided"),
            MyError::FileNotFound(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for MyError {}