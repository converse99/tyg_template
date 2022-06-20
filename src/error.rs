use std::fmt;
use std::io;

/// A specialized [`Result`] type for use with the command line template.
///
/// This typedef is generally used to simplify [`Result`] usage when using the template
///
/// # Examples
/// ```
/// use tyg_template::{Result, Error, option_err};
///
/// fn generate_error() -> Result<()> {
///     let value = None;
///     let message = "This is a test error messaage";
///     value.ok_or_else(|| option_err!("{}", message))?;
///     Ok(())
/// }
///
/// let result = generate_error();
///
/// assert!(result.is_err());
/// println!("{:?}", result);
/// ```
pub type Result<T> = std::result::Result<T, Error>;

//  option_err macro
/// Macro to prepare a disclosed error when transforming an `Option<T>` into a `Result<T, E>`, that
/// can be handled by the calling context using the '?' operator or by simply returning it.
///
/// The macro creates an error message containing the name of the source file and the
/// location in the source where the error occured. This is ideal for debugging purposes.
///
/// This macro is particularly useful when using
/// [`ok_or_else()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else)
///
/// # Examples
/// ```
/// use tyg_template::{Result, Error, option_err};
///
/// fn generate_error() -> Result<()> {
///     let value = None;
///     let message = "This is a test error messaage";
///     value.ok_or_else(|| option_err!("{}", message))?;
///     Ok(())
/// }
///
/// let result = generate_error();
///
/// assert!(result.is_err());
/// println!("{:?}", result);
/// ```
#[macro_export]
macro_rules! option_err {
    ( $( $arg:expr),+ ) => {{
        let details = format!( $($arg,)+ );
        let error_text = format!("{}:{}:{}: {}", file!(), line!(), column!(), details);
        Error::Error(error_text)
    }};
}

//  option_err_bare macro
/// Macro to prepare a non-disclosed error when transforming an `Option<T>` into a `Result<T, E>`, that
/// can be handled by the calling context using the '?' operator or by simply returning it.
///
/// The macro creates an error message, but does not disclose the location where the
/// error occured. This is ideal for for presenting the end user with the cause of the error.
///
/// This macro is particularly useful when using
/// [`ok_or_else()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else)
///
/// If the code is compiled with the 'disclose' feature enabled, the macro creates an error message
/// containing the name of the source file and the location in the source where the error occured.
/// It is equivalent to the macro called option_err
///
/// # Examples
/// ```
/// use tyg_template::{Result, Error, option_err_bare};
///
/// fn generate_error() -> Result<()> {
///     let value = None;
///     let message = "This is a test error messaage";
///     value.ok_or_else(|| option_err_bare!("{}", message))?;
///     Ok(())
/// }
///
/// let result = generate_error();
///
/// assert!(result.is_err());
/// println!("{:?}", result);
/// ```
#[macro_export]
macro_rules! option_err_bare {
    ( $( $arg:expr),+ ) => {{
        if cfg!(feature = "disclose") {
            let details = format!( $($arg,)+ );
            let error_text = format!("{}:{}:{}: {}", file!(), line!(), column!(), details);
            Error::Error(error_text)
        } else {
            let details = format!( $($arg,)+ );
            Error::Error(details)
        }
    }};
}

//  result_err macro
/// Macro to prepare a disclosed error of type `Result<T, E>` that can be handled by the calling
/// context either by using the '?' operator or by simply returning it.
///
/// The macro creates an error message containing the name of the source file and the location in
/// the source where the error occured. This is ideal for debugging purposes.
///
/// This macro is particularly useful when using
/// [`or_else()`](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.or_else)
///
/// # Examples
/// ```
/// use tyg_template::{Result, Error, result_err};
///
/// fn generate_error() -> Result<()> {
///     "NaN".parse::<u32>().or_else(|e| result_err!("Oh dear - {}", e))?;
///     Ok(())
/// }
///
/// let result = generate_error();
///
/// assert!(result.is_err());
/// println!("{:?}", result);
/// ```
#[macro_export]
macro_rules! result_err {
    ( $( $arg:expr),+ ) => {{
        let details = format!( $($arg,)+ );
        let error_text = format!("{}:{}:{}: {}", file!(), line!(), column!(), details);
        Err(Error::Error(error_text))
    }};
}

//  result_err_bare macro
/// Macro to prepare a non-disclosed error of type `Result<T, E>` that can be handled by the calling
/// context either by using the '?' operator or by simply returning it.
///
/// The macro creates an error message, but does not disclose the location where the error occured.
/// This is ideal for for presenting the end user with the cause of the error.
///
/// This macro is particularly useful when using
/// [`or_else()`](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.or_else)
///
/// If the code is compiled with the 'disclose' feature enabled, the macro creates an error message
/// containing the name of the source file and the location in the source where the error occured.
/// It is equivalent to the macro called result_err
///
/// # Examples
/// ```
/// use tyg_template::{Result, Error, result_err_bare};
///
/// fn generate_error() -> Result<()> {
///     "NaN".parse::<u32>().or_else(|e| result_err_bare!("Oh dear - {}", e))?;
///     Ok(())
/// }
///
/// let result = generate_error();
///
/// assert!(result.is_err());
/// println!("{:?}", result);
/// ```
#[macro_export]
macro_rules! result_err_bare {
    ( $( $arg:expr),+ ) => {{
        if cfg!(feature = "disclose") {
            let details = format!( $($arg,)+ );
            let error_text = format!("{}:{}:{}: {}", file!(), line!(), column!(), details);
            Err(Error::Error(error_text))
        } else {
            let details = format!( $($arg,)+ );
            Err(Error::Error(details))
        }
    }};
}

macro_rules! formatter {
    ( $self:expr, $f:expr ) => {
        match *$self {
            Error::Error(ref e) => write!($f, "{}", e),
            Error::File(ref e) => e.fmt($f),
        }
    };
}

// pub enum error
/// The template error enumeration is used to define the various error types that can be handled by
/// the template crate. The only variant that is required for basic operation is the Error variant.
/// The File variant has been included for demonstration purposes.
///
/// Remember the following when adding new variants:
/// - Add the variant to the formatter macro.
/// - Implement From for the new variant.
///
/// For an idea on how to implement error variants have a look at the source code here.
pub enum Error {
    /// Custom Error of type `String`.
    Error(String),
    /// Error of type `io::Error`.
    File(io::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        formatter!(self, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        formatter!(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Error(_) => None,
            Error::File(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::File(err)
    }
}
