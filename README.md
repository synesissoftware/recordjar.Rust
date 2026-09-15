# recordjar.Rust <!-- omit in toc -->

Record-Jar structured text database reader for Rust — part of the cross-language **Open-RJ** family.

![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/recordjar.Rust.svg)](https://github.com/synesissoftware/recordjar.Rust/releases/latest)
[![Last Commit](https://img.shields.io/github/last-commit/synesissoftware/recordjar.Rust)](https://github.com/synesissoftware/recordjar.Rust/commits/master)
![MSRV](https://img.shields.io/badge/MSRV-1.74-lightgrey)
[![CI](https://github.com/synesissoftware/recordjar.Rust/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/recordjar.Rust/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/recordjar/badge.svg)](https://docs.rs/recordjar)


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Installation](#installation)
- [Components](#components)
- [Examples](#examples)
- [Project Information](#project-information)
  - [Where to get help](#where-to-get-help)
  - [Contribution guidelines](#contribution-guidelines)
  - [Minimum Supported Rust Version (MSRV)](#minimum-supported-rust-version-msrv)
  - [Dependencies](#dependencies)
    - [Dev Dependencies](#dev-dependencies)
  - [Related projects](#related-projects)
  - [License](#license)


## Introduction

**recordjar** reads Record-JAR databases: text files (or in-memory buffers) of
records separated by `%%` lines, each record holding `name: value` fields.
Continuation lines, comments, and the implemented parse-time flags are
supported by the memory parser; file I/O, field aliases, and field lookup
remain planned.


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
recordjar = { version = "0.0.2" }
```


## Components

| Type | Purpose |
| --- | --- |
| [`Database`](https://docs.rs/recordjar/latest/recordjar/struct.Database.html) | Parsed Record-JAR database |
| [`Record`](https://docs.rs/recordjar/latest/recordjar/struct.Record.html) | Record with comment and fields |
| [`Field`](https://docs.rs/recordjar/latest/recordjar/struct.Field.html) | Single `name: value` field |
| [`ParseFlags`](https://docs.rs/recordjar/latest/recordjar/struct.ParseFlags.html) | Parse-time behaviour flags |
| [`Error`](https://docs.rs/recordjar/latest/recordjar/enum.Error.html) | Operation and parse errors |
| [`ParseErrorKind`](https://docs.rs/recordjar/latest/recordjar/enum.ParseErrorKind.html) | Parse error classification |
| [`ParseErrorDetail`](https://docs.rs/recordjar/latest/recordjar/struct.ParseErrorDetail.html) | Line/column parse failure detail |

Memory parsing is implemented for records, fields, comments, continuations,
parse errors, blank-record elision, field ordering, and single-record mode.
File I/O, field aliases, and field lookup remain under development.

| Function | Purpose |
| --- | --- |
| `Database::from_str()` | Parse from memory |
| `Database::from_path()` | Parse from file path (not yet implemented) |
| `Record::find_field_by_name()` | Field lookup (not yet implemented) |


## Examples

See [**EXAMPLES.md**](./EXAMPLES.md) for the available examples.


## Project Information


### Where to get help

[GitHub Issues](https://github.com/synesissoftware/recordjar.Rust/issues)


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/recordjar.Rust.


### Minimum Supported Rust Version (MSRV)

The declared Minimum Supported Rust Version (MSRV) for **recordjar.Rust** is **1.74**.

This MSRV guarantee applies to the library crate itself, its runtime dependencies (`[dependencies]`), and its build dependencies (`[build-dependencies]`). Downstream consumers compiling this crate as a dependency are guaranteed that it builds cleanly on the declared MSRV toolchain.

Development dependencies (`[dev-dependencies]`, such as benchmarking frameworks like **criterion**) may require newer Rust toolchains for local development or performance testing. These dev-dependencies are never fetched or compiled by downstream consumers and do not affect the library's MSRV guarantee.


### Dependencies

None.


#### Dev Dependencies

* [**criterion**](https://crates.io/crates/criterion);
* [**shwild**](https://crates.io/crates/shwild);
* [**test_help-rs**](https://crates.io/crates/test_help-rs);


### Related projects

* [**openrj**](https://github.com/synesissoftware/openrj);
* [**recordjar.Rust**](https://github.com/synesissoftware/recordjar.Rust);


### License

**recordjar.Rust** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
