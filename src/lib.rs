/* /////////////////////////////////////////////////////////////////////////
 * File:    src/lib.rs
 *
 * Purpose: Primary implementation file for recordjar.Rust.
 *
 * Created: 20th August 2026
 * Updated: 20th August 2026
 *
 * Home:    https://github.com/synesissoftware/recordjar.Rust
 *
 * Copyright (c) 2019-2026, Matthew Wilson and Synesis Information Systems
 * Copyright (c) 2004-2019, Matthew Wilson and Synesis Software
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.
 * - Neither the name of the copyright holder nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ////////////////////////////////////////////////////////////////////// */


//! Record-Jar structured text database reader for Rust.
//!
//! **recordjar** parses Record-JAR files: records separated by `%%`
//! lines, each holding `name: value` fields. Part of the cross-language
//! **Open-RJ** family (see [**openrj**][orj] for the C implementation).
//!
//! # Components
//!
//! ## Types
//!
//! * [`Database`] — a parsed Record-JAR database;
//! * [`Record`] — a record with comment and fields;
//! * [`Field`] — a single `name: value` pair;
//! * [`ParseFlags`] — parse-time behaviour flags;
//! * [`Error`] — operation and parse errors;
//! * [`ParseErrorDetail`] — line/column parse failure detail;
//! * [`ParseErrorKind`] — parse error classification;
//!
//! ## Functions
//!
//! * [`Database::from_str()`] — parse from memory;
//! * [`Database::from_path()`] — parse from a file path;
//!
//! [orj]: https://github.com/synesissoftware/openrj


mod error;
mod flags;
mod types;


pub use error::{
    Error,
    ParseErrorDetail,
    ParseErrorKind,
    Result,
};
pub use flags::ParseFlags;
pub use types::{
    Database,
    Field,
    Record,
};
