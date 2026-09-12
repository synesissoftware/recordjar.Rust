/* /////////////////////////////////////////////////////////////////////////
 * File:    src/parse.rs
 *
 * Purpose: Record-JAR memory parser for recordjar.Rust.
 *
 * Created: 20th August 2026
 * Updated: 11th September 2026
 *
 * ////////////////////////////////////////////////////////////////////// */


#[rustfmt::skip]
use std::{
    mem as std_mem,
};

use crate::{
    error::{
        Error,
        ParseErrorDetail,
        ParseErrorKind,
        Result,
    },
    flags::ParseFlags,
    types::{
        Database,
        Field,
        Record,
    },
};


/// Parses a Record-JAR database from `content`.
pub fn parse_database(
    content : &str,
    flags : ParseFlags,
) -> Result<Database> {
    if content.is_empty() {
        return Err(Error::NoRecords);
    }

    let normalized = normalize_lines(content)?;

    build_database(normalized, flags)
}


struct NormalizedLines {
    lines :     Vec<String>,
    num_lines : usize,
}


fn normalize_lines(content : &str) -> Result<NormalizedLines> {
    let bytes = content.as_bytes();
    let end = bytes.len();
    let mut begin : usize = 0;
    let mut start : usize = 0;
    let mut dest_line = String::new();
    let mut output_lines : Vec<String> = Vec::new();
    let mut last_char : u8 = 0;
    let mut num_lines_to_insert : i32 = 0;
    let mut num_initial_ws_to_skip : i32 = 0;
    let mut line : u32 = 0;
    let mut column : u32 = 0;
    let mut leading_comment_chars : i32 = 0;
    let mut is_in_continuation = false;
    let mut is_in_comment_line = false;
    let mut field_length : i32 = 0;
    let mut num_fields_this_record : i32 = 0;

    while begin < end {
        let this_char = bytes[begin];

        match this_char {
            b'\n' => {
                let mut cch = begin - start;
                if last_char == b'\r' && cch > 0 {
                    cch -= 1;
                }

                if cch > 0 {
                    dest_line.push_str(&content[start..start + cch]);
                }

                if last_char == b'\\' {
                    dest_line.pop();
                    is_in_continuation = true;
                    field_length += column as i32;
                } else {
                    if is_in_continuation {
                        field_length += column as i32;
                    } else {
                        field_length = column as i32;
                    }

                    if field_length > 0 && !is_in_comment_line && num_initial_ws_to_skip == 0 {
                        num_fields_this_record += 1;
                    }
                    field_length = 0;
                    is_in_continuation = false;
                    is_in_comment_line = false;

                    output_lines.push(dest_line.clone());
                    dest_line.clear();
                    num_lines_to_insert = 0;
                }

                start = begin + 1;
                num_initial_ws_to_skip = 0;
                line += 1;
                column = 0;
                leading_comment_chars = 0;
            },
            b' ' | b'\r' | b'\t' => {
                if last_char == b'\n' || num_initial_ws_to_skip > 0 {
                    num_initial_ws_to_skip += 1;
                    start = begin + 1;
                } else if last_char == b'\\' {
                    let mut j = begin;
                    let mut trailing = true;

                    while j < end {
                        match bytes[j] {
                            b' ' | b'\r' | b'\t' => j += 1,
                            b'\n' => break,
                            _ => {
                                trailing = false;
                                break;
                            },
                        }
                    }

                    if trailing {
                        let cch = begin - start - 1;
                        if cch > 0 {
                            dest_line.push_str(&content[start..start + cch]);
                        }
                        begin = j;
                        start = j + 1;
                        line += 1;
                        column = 0;
                        leading_comment_chars = 0;
                        last_char = b'\n';
                        continue;
                    }
                }

                if leading_comment_chars == 1 {
                    leading_comment_chars = 0;
                    num_initial_ws_to_skip = 0;
                }
            },
            b'%' => {
                if !is_in_comment_line && (column == 0 || leading_comment_chars > 0) {
                    leading_comment_chars += 1;

                    if leading_comment_chars > 1 {
                        num_fields_this_record = 0;
                        is_in_comment_line = true;

                        if is_in_continuation {
                            return Err(parse_error(line, column, ParseErrorKind::RecordSeparatorInContinuation));
                        }
                    }
                }
            },
            b':' => {
                if column == 0 || num_initial_ws_to_skip > 0 {
                    return Err(parse_error(line, column, ParseErrorKind::InvalidFieldName));
                }
            },
            _ => {
                num_initial_ws_to_skip = 0;
            },
        }

        last_char = this_char;

        if this_char == b'\n' {
            column = 0;
            leading_comment_chars = 0;
        } else {
            column += 1;
        }

        begin += 1;
    }

    if last_char != b'\n' && last_char != 0 {
        return Err(parse_error(line, column, ParseErrorKind::UnfinishedLine));
    }

    if num_fields_this_record != 0 {
        return Err(parse_error(line, column, ParseErrorKind::UnfinishedRecord));
    }

    if num_lines_to_insert != 0 || is_in_continuation {
        return Err(parse_error(line, column, ParseErrorKind::UnfinishedField));
    }

    Ok(NormalizedLines {
        num_lines : line as usize,
        lines :     output_lines,
    })
}


fn build_database(
    normalized : NormalizedLines,
    flags : ParseFlags,
) -> Result<Database> {
    let mut records : Vec<Record> = Vec::new();
    let mut flat_fields : Vec<Field> = Vec::new();
    let mut pending_fields : Vec<Field> = Vec::new();
    let elide_blank = flags.contains(ParseFlags::ELIDE_BLANK_RECORDS);

    for line in normalized.lines {
        if line.is_empty() {
            continue;
        }

        if is_comment_line(&line) {
            let comment = parse_comment(&line);
            push_record(
                &mut records,
                &mut flat_fields,
                comment,
                std_mem::take(&mut pending_fields),
                elide_blank,
            );
        } else {
            pending_fields.push(parse_field_line(&line)?);
        }
    }

    if flags.contains(ParseFlags::ORDER_FIELDS) {
        for record in &mut records {
            record.fields.sort_by(|left, right| left.name().cmp(right.name()));
        }
        flat_fields = records.iter().flat_map(|record| record.fields.clone()).collect();
    }

    if flags.contains(ParseFlags::FORCE_ALL_FIELDS_INTO_1_RECORD) {
        let mut all_fields : Vec<Field> = Vec::new();
        let mut merged_comment = String::new();

        for record in records {
            if !merged_comment.is_empty() && !record.comment.is_empty() {
                merged_comment.push('\n');
            }
            merged_comment.push_str(record.comment());
            all_fields.extend(record.fields);
        }

        records = vec![Record {
            comment : merged_comment,
            fields :  all_fields.clone(),
        }];
        flat_fields = all_fields;
    }

    if records.is_empty() {
        return Err(Error::NoRecords);
    }

    let num_fields = flat_fields.len();

    Ok(Database {
        flags,
        records,
        fields : flat_fields,
        num_lines : normalized.num_lines,
        num_fields,
    })
}


impl Database {
    /// Parses a Record-JAR database from a string slice.
    pub fn from_str(
        content : &str,
        flags : ParseFlags,
    ) -> Result<Self> {
        parse_database(content, flags)
    }
}


fn push_record(
    records : &mut Vec<Record>,
    flat_fields : &mut Vec<Field>,
    comment : String,
    fields : Vec<Field>,
    elide_blank : bool,
) {
    if elide_blank && fields.is_empty() {
        return;
    }

    flat_fields.extend(fields.iter().cloned());
    records.push(Record {
        comment,
        fields,
    });
}


fn is_comment_line(text : &str) -> bool {
    text.starts_with("%%")
}


fn parse_comment(text : &str) -> String {
    let mut comment = text.get(2..).unwrap_or("").to_string();

    trim_leading_ws(&mut comment);
    trim_trailing_ws(&mut comment);

    comment
}


fn parse_field_line(text : &str) -> Result<Field> {
    let end = trim_trailing_end(text);
    let trimmed = &text[..end];
    let colon = trimmed.find(':').unwrap_or(trimmed.len());
    let (name_part, value_part) = if colon < trimmed.len() {
        (&trimmed[..trim_trailing_end(&trimmed[..colon])], &trimmed[colon + 1..])
    } else {
        (trimmed, "")
    };

    let name = trim_ws(name_part);
    if name.is_empty() {
        return Err(parse_error(0, 0, ParseErrorKind::InvalidFieldName));
    }

    let value = trim_ws(value_part);

    Ok(Field {
        name :  name.to_string(),
        value : value.to_string(),
    })
}


fn trim_trailing_end(text : &str) -> usize {
    let mut end = text.len();

    while end > 0 {
        match text.as_bytes()[end - 1] {
            b' ' | b'\t' => end -= 1,
            _ => break,
        }
    }

    end
}


fn trim_ws(s : &str) -> &str {
    trim_leading_ws_str(trim_trailing_ws_str(s))
}


fn trim_leading_ws(s : &mut String) {
    *s = trim_leading_ws_str(s.as_str()).to_string();
}


fn trim_trailing_ws(s : &mut String) {
    *s = trim_trailing_ws_str(s.as_str()).to_string();
}


fn trim_leading_ws_str(s : &str) -> &str {
    s.trim_start_matches([' ', '\t'])
}


fn trim_trailing_ws_str(s : &str) -> &str {
    s.trim_end_matches([' ', '\t'])
}


fn parse_error(
    line : u32,
    column : u32,
    kind : ParseErrorKind,
) -> Error {
    Error::ParseError {
        detail : ParseErrorDetail {
            line,
            column,
            kind,
        },
    }
}


#[cfg(test)]
mod TEST_parse {
    #![allow(non_snake_case)]

    use super::parse_database;
    use crate::{
        error::ParseErrorKind,
        flags::ParseFlags,
    };


    const CATS_AND_DOGS : &str = "\
%% Sample Open-RJ database - Cats and Dogs\n\
%% Created:   28th September 2004\n\
%% Updated:   29th September 2004\n\
Name:      Barney\n\
Species:   Dog\n\
Breed:     Bijon \\\n\
           Frieze\n\
%%\n\
Name:      Elsa\n\
Species:   Dog\n\
Breed:     Mixed\n\
%%\n\
Name:      Fluffy Kitten\n\
Species:   Cat\n\
%%\n\
Name:      Moet\n\
Species:   Dog\n\
Breed:     Boxer\n\
%%\n\
Name:      Rebel\n\
Species:   Dog\n\
Breed:     German \\\n\
           Shepherd\n\
%%\n\
Name:      Sparky\n\
Species:   Cat\n\
%%\n\
";

    const INVALID_FIELD_NAME : &str = "\
%% Sample Open-RJ database - Cats and Dogs\n\
%% Created:   28th September 2004\n\
%% Updated:   29th September 2004\n\
Name:      Barney\n\
Species:   Dog\n\
Breed:     Bijon \\\n\
           Frieze\n\
%%\n\
  : \n\
Species:   Cat\n\
    :\n\
    : \n\
%%\n\
Name:      Moet\n\
Species:   Dog\n\
Breed:     Boxer\n\
%%\n\
";

    const ELIDE_BLANK : &str = "\
Name: A\n\
%%\n\
%%\n\
Name: B\n\
%%\n\
";

    #[test]
    fn TEST_CREATE_FROM_MEMORY_COUNTS() {
        let db = parse_database(CATS_AND_DOGS, ParseFlags::empty()).expect("parse");

        assert_eq!(9, db.num_records());
        assert_eq!(16, db.num_fields());
    }

    #[test]
    fn TEST_INVALID_FIELD_NAME_PARSE_ERROR() {
        let err = parse_database(INVALID_FIELD_NAME, ParseFlags::empty()).expect_err("parse");

        match err {
            crate::error::Error::ParseError {
                detail,
            } => {
                assert_eq!(ParseErrorKind::InvalidFieldName, detail.kind);
                assert_eq!(8, detail.line);
            },
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn TEST_ELIDE_BLANK_RECORDS() {
        let db = parse_database(ELIDE_BLANK, ParseFlags::ELIDE_BLANK_RECORDS).expect("parse");

        assert_eq!(2, db.num_records());
    }

    #[test]
    fn TEST_CONTINUATION_FIELD_VALUE() {
        let db = parse_database(CATS_AND_DOGS, ParseFlags::empty()).expect("parse");
        let record = db.record(3).expect("record");
        let breed = record.field(2).expect("breed");

        assert_eq!("Breed", breed.name());
        assert_eq!("Bijon Frieze", breed.value());
    }
}
