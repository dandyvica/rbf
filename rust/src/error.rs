use std::error::Error;
use std::fmt;

/// Macro which builds a vector of Record data fields.
///
/// # Example
#[macro_export]
macro_rules! error_check {
    ($cond:expr) => {{
        match $cond {
            Ok(v) => v,
            Err(e) => {
                println!("error: {}", e);
                ::std::process::exit(2);
            }
        }
    }};
}

/// Alias for a `Result` with the error type `error::RbfError`.
pub type Result<T> = ::std::result::Result<T, RbfError>;

#[derive(Debug)]
pub enum RbfError {
    /// The field type specified in layout is not existing.
    NoFieldTypeFound,

    /// mainly, file not found
    ErrorOpeningLayoutFile(String, ::std::io::Error),

    /// error when interpreting XML
    ErrorReadingLayoutFile(String, ::xml::reader::Error),

    /// no field type defined in the layout file
    ErrorLayoutNoFieldType(String, String, String),
}

impl Error for RbfError {
    fn description(&self) -> &str {
        match *self {
            RbfError::NoFieldTypeFound => "The field type specified in layout is not existing",
            RbfError::ErrorOpeningLayoutFile(_, _) => "the layout file could not be opened",
            RbfError::ErrorReadingLayoutFile(_, _) => {
                "an error occured when reading the layout file"
            }
            RbfError::ErrorLayoutNoFieldType(_, _, _) => {
                "an error occured when reading the layout file"
            }
        }
    }
}

impl fmt::Display for RbfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RbfError::NoFieldTypeFound => write!(f, ""),
            RbfError::ErrorOpeningLayoutFile(ref file, ref e) => write!(
                f,
                "the layout file <{}> could not be opened, io error={}",
                file, e
            ),
            RbfError::ErrorReadingLayoutFile(ref file, ref e) => write!(
                f,
                "an error occured when reading the layout file <{}>, xml error={}",
                file, e
            ),
            RbfError::ErrorLayoutNoFieldType(ref file, ref f_name, ref f_type) => write!(
                f,
                "no field type <{}> for field name <{}> found in layout file <{}>",
                f_type, f_name, file
            ),
        }
    }
}

/*panic!("couldn't open {}: {}, current directory is: {}",
xml_file, why.description(), env::current_dir().unwrap().display()),*/
