pub mod absolute;

#[cfg(windows)]
pub const PATH_SEP_STR: &str = r"\";
#[cfg(windows)]
pub const PATH_SEP_CHAR: char = '\\';

#[cfg(unix)]
pub const PATH_SEP_STR: &str = "/";
#[cfg(unix)]
pub const PATH_SEP_CHAR: char = '/';
