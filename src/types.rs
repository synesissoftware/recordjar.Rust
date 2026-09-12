/* /////////////////////////////////////////////////////////////////////////
 * File:    src/types.rs
 *
 * Purpose: Database, record, and field types for recordjar.Rust.
 *
 * Created: 20th August 2026
 * Updated: 11th September 2026
 *
 * ////////////////////////////////////////////////////////////////////// */


#[rustfmt::skip]
use std::{
    path as std_path,
};

use crate::{
    error::{
        Error,
        Result,
    },
    flags::ParseFlags,
};


/// A single `name: value` field within a record.
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct Field {
    pub(crate) name :  String,
    pub(crate) value : String,
}


/// A Record-JAR record: a comment and an ordered list of fields.
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct Record {
    pub(crate) comment : String,
    pub(crate) fields :  Vec<Field>,
}


/// A parsed Record-JAR database.
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct Database {
    pub(crate) flags :      ParseFlags,
    pub(crate) records :    Vec<Record>,
    pub(crate) fields :     Vec<Field>,
    pub(crate) num_lines :  usize,
    pub(crate) num_fields : usize,
}


impl Field {
    /// Returns the field name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the field value.
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}


impl Record {
    /// Returns the record comment (text from `%%` lines before fields).
    pub fn comment(&self) -> &str {
        self.comment.as_str()
    }

    /// Returns the number of fields in this record.
    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }

    /// Returns the field at `index`, or [`Error::InvalidIndex`].
    pub fn field(
        &self,
        index : usize,
    ) -> Result<&Field> {
        self.fields.get(index).ok_or(Error::InvalidIndex)
    }

    /// Finds a field by `name`, optionally matching `value`.
    ///
    /// When `value` is `None`, returns the first field with the given name.
    pub fn find_field_by_name(
        &self,
        name : &str,
        value : Option<&str>,
    ) -> Option<&Field> {
        let _ = (name, value);

        todo!("field lookup not yet implemented")
    }

    /// Finds the next field after `after`, optionally filtered by name
    /// and/or value.
    pub fn find_next_field(
        &self,
        after : Option<&Field>,
        name : Option<&str>,
        value : Option<&str>,
    ) -> Option<&Field> {
        let _ = (after, name, value);

        todo!("field lookup not yet implemented")
    }
}


impl Database {
    /// Parses a Record-JAR database from a file path.
    pub fn from_path(
        path : &std_path::Path,
        _flags : ParseFlags,
    ) -> Result<Self> {
        let _ = path;

        todo!("file I/O not yet implemented")
    }

    /// Returns the parse flags used to create this database.
    pub fn flags(&self) -> ParseFlags {
        self.flags
    }

    /// Returns the number of lines in the source content.
    pub fn num_lines(&self) -> usize {
        self.num_lines
    }

    /// Returns the total number of fields across all records.
    pub fn num_fields(&self) -> usize {
        self.num_fields
    }

    /// Returns the number of records in this database.
    pub fn num_records(&self) -> usize {
        self.records.len()
    }

    /// Returns the record at `index`, or [`Error::InvalidIndex`].
    pub fn record(
        &self,
        index : usize,
    ) -> Result<&Record> {
        self.records.get(index).ok_or(Error::InvalidIndex)
    }

    /// Returns the global field at `index`, or [`Error::InvalidIndex`].
    pub fn field(
        &self,
        index : usize,
    ) -> Result<&Field> {
        self.fields.get(index).ok_or(Error::InvalidIndex)
    }
}
