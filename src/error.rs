/* /////////////////////////////////////////////////////////////////////////
 * File:    src/error.rs
 *
 * Purpose: Error types for recordjar.Rust.
 *
 * Created: 20th August 2026
 * Updated: 20th August 2026
 *
 * ////////////////////////////////////////////////////////////////////// */


#[rustfmt::skip]
use std::{
    error as std_error,
    fmt as std_fmt,
    result as std_result,
};


/// Result type alias for recordjar operations.
pub type Result<T> = std_result::Result<T, Error>;


/// Parsing error kind.
///
/// Equivalent to **openrj** `ORJ_PARSE_ERROR` constants.
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum ParseErrorKind {
    /// A record separator was encountered during a content line
    /// continuation.
    RecordSeparatorInContinuation,
    /// The last line in the database was not terminated by a line-feed.
    UnfinishedLine,
    /// The last field in the database file was not well-formed.
    UnfinishedField,
    /// The last record was not terminated by a record separator.
    UnfinishedRecord,
    /// The field name was not valid.
    InvalidFieldName,
}


/// Location and kind of a parse failure.
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct ParseErrorDetail {
    /// One-based line number of the error.
    pub line :   u32,
    /// One-based column number of the error.
    pub column : u32,
    /// The parse error kind.
    pub kind :   ParseErrorKind,
}


/// Top-level error type for recordjar operations.
///
/// Equivalent to **openrj** `ORJRC` return codes.
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Error {
    /// The given file does not exist, or cannot be accessed.
    CannotOpenJarFile,
    /// The database file contained no records.
    NoRecords,
    /// Memory exhaustion.
    OutOfMemory,
    /// A read operation failed.
    BadFileRead,
    /// Parsing failed due to a syntax error.
    ParseError {
        /// Parse error detail.
        detail : ParseErrorDetail,
    },
    /// An invalid index was specified.
    InvalidIndex,
    /// An unexpected condition was encountered.
    Unexpected,
    /// The database file contained invalid content.
    InvalidContent,
}


impl ParseErrorKind {
    /// English message for this parse error kind.
    pub fn message(self) -> &'static str {
        match self {
            Self::RecordSeparatorInContinuation => {
                "A record separator was encountered during a content line \
                 continuation"
            },
            Self::UnfinishedLine => {
                "The last line in the database was not terminated by a \
                 line-feed"
            },
            Self::UnfinishedField => "The last field in the database file was not well-formed",
            Self::UnfinishedRecord => {
                "The last record in the database file was not terminated \
                 by a record separator"
            },
            Self::InvalidFieldName => "The field name was not valid",
        }
    }
}


impl std_fmt::Display for ParseErrorKind {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        f.write_str(self.message())
    }
}


impl std_fmt::Display for Error {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        let message = match self {
            Self::CannotOpenJarFile => "The given file does not exist, or cannot be accessed",
            Self::NoRecords => "The database file contained no records",
            Self::OutOfMemory => "The API suffered memory exhaustion",
            Self::BadFileRead => "A read operation failed",
            Self::ParseError {
                detail,
            } => {
                return write!(
                    f,
                    "Parsing of the database file failed due to a syntax \
                     error: {}",
                    detail.kind,
                );
            },
            Self::InvalidIndex => "An invalid index was specified",
            Self::Unexpected => "An unexpected condition was encountered",
            Self::InvalidContent => "The database file contained invalid content",
        };

        f.write_str(message)
    }
}


impl std_error::Error for Error {
}
