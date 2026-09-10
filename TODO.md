# recordjar.Rust - TODO <!-- omit in toc -->


## Table of Contents <!-- omit in toc -->

- [Functional improvements](#functional-improvements)
- [Performance improvements](#performance-improvements)
- [Packaging improvements](#packaging-improvements)


## Functional improvements

* [x] ~~~public types (`Database`, `Record`, `Field`, flags, errors)~~~ - ✅;
* [x] ~~~memory parser (records, fields, continuations, comments)~~~ - ✅;
* [ ] parse flags and field aliases;
* [ ] field lookup API;
* [ ] file I/O;
* [ ] unit tests (port **openrj** corpus);
* [ ] rename test functions to Synesis canonical **RUST_TEST_NAMING** (`TEST_` + `SHOUTING_SNAKE_CASE`; construct names such as `ParseFlags` and `Database` preserve PascalCase; see **rust-standards** and `scripts/check_test_names.py`) — current parser tests use mixed case (e.g. `TEST_create_from_memory_counts` → `TEST_CREATE_FROM_MEMORY_COUNTS`);
* [ ] examples;


## Performance improvements

* \<none>


## Packaging improvements

* [ ] crates.io publish metadata review;


<!-- ########################### end of file ########################### -->
