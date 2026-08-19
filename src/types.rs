/* /////////////////////////////////////////////////////////////////////////
 * File:    src/types.rs
 *
 * Purpose: Database, record, and field types for recordjar.Rust.
 *
 * Created: 20th August 2026
 * Updated: 20th August 2026
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
#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct Field<'a> {
    name :  &'a str,
    value : &'a str,
}


/// A Record-JAR record: a comment and an ordered list of fields.
#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct Record<'a> {
    comment : &'a str,
    fields :  Vec<Field<'a>>,
}


/// A parsed Record-JAR database.
#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct Database<'a> {
    source :     &'a str,
    flags :      ParseFlags,
    records :    Vec<Record<'a>>,
    num_lines :  usize,
    num_fields : usize,
}


impl<'a> Field<'a> {
    /// Returns the field name.
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Returns the field value.
    pub fn value(&self) -> &'a str {
        self.value
    }
}


impl<'a> Record<'a> {
    /// Returns the record comment (text from `%%` lines before fields).
    pub fn comment(&self) -> &'a str {
        self.comment
    }

    /// Returns the number of fields in this record.
    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }

    /// Returns the field at `index`, or [`Error::InvalidIndex`].
    pub fn field(
        &self,
        index : usize,
    ) -> Result<&Field<'a>> {
        self.fields.get(index).ok_or(Error::InvalidIndex)
    }

    /// Finds a field by `name`, optionally matching `value`.
    ///
    /// When `value` is `None`, returns the first field with the given name.
    pub fn find_field_by_name(
        &self,
        name : &str,
        value : Option<&str>,
    ) -> Option<&Field<'a>> {
        let _ = (name, value);

        todo!("field lookup not yet implemented")
    }

    /// Finds the next field after `after`, optionally filtered by name
    /// and/or value.
    pub fn find_next_field(
        &self,
        after : Option<&Field<'a>>,
        name : Option<&str>,
        value : Option<&str>,
    ) -> Option<&Field<'a>> {
        let _ = (after, name, value);

        todo!("field lookup not yet implemented")
    }
}


impl<'a> Database<'a> {
    /// Parses a Record-JAR database from a string slice.
    pub fn from_str(
        content : &'a str,
        flags : ParseFlags,
    ) -> Result<Self> {
        let _ = (content, flags);

        todo!("parser not yet implemented")
    }

    /// Parses a Record-JAR database from a file path.
    pub fn from_path(
        path : &std_path::Path,
        flags : ParseFlags,
    ) -> Result<Self> {
        let _ = (path, flags);

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
    ) -> Result<&Record<'a>> {
        self.records.get(index).ok_or(Error::InvalidIndex)
    }

    /// Returns the global field at `index`, or [`Error::InvalidIndex`].
    pub fn field(
        &self,
        index : usize,
    ) -> Result<&Field<'a>> {
        let _ = index;

        todo!("global field index not yet implemented")
    }
}
