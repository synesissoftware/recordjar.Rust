/* /////////////////////////////////////////////////////////////////////////
 * File:    src/flags.rs
 *
 * Purpose: Parse flags for recordjar.Rust.
 *
 * Created: 20th August 2026
 * Updated: 20th August 2026
 *
 * ////////////////////////////////////////////////////////////////////// */


#[rustfmt::skip]
use std::{
    ops as std_ops,
};


/// Flags passed when parsing a Record-JAR database.
///
/// Equivalent to **openrj** `ORJ_FLAG_*` constants.
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
pub struct ParseFlags(u32);


impl ParseFlags {
    /// Causes blank records to be ignored.
    pub const ELIDE_BLANK_RECORDS : Self = Self(0x0002);
    /// Causes record separators to be ignored; all fields in one record.
    pub const FORCE_ALL_FIELDS_INTO_1_RECORD : Self = Self(0x0008);
    /// Ignores case when looking up field names.
    pub const IGNORE_CASE_ON_LOOKUP : Self = Self(0x0004);
    /// Suppresses field identifier reinterpretation (aliases).
    pub const NO_REINTERPRET_FIELD_IDS : Self = Self(0x0100);
    /// Arranges the fields in alphabetical order within each record.
    pub const ORDER_FIELDS : Self = Self(0x0001);

    /// Creates an empty flag set.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns the raw flag bits.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Returns whether all bits in `flag` are set in `self`.
    pub const fn contains(
        self,
        flag : Self,
    ) -> bool {
        (self.0 & flag.0) == flag.0
    }

    /// Returns the union of `self` and `other`.
    pub const fn union(
        self,
        other : Self,
    ) -> Self {
        Self(self.0 | other.0)
    }
}


impl std_ops::BitOr for ParseFlags {
    type Output = Self;

    fn bitor(
        self,
        rhs : Self,
    ) -> Self {
        self.union(rhs)
    }
}


impl std_ops::BitOrAssign for ParseFlags {
    fn bitor_assign(
        &mut self,
        rhs : Self,
    ) {
        *self = self.union(rhs);
    }
}


#[cfg(test)]
mod TEST_ParseFlags {
    #![allow(non_snake_case)]

    use super::ParseFlags;


    #[test]
    fn TEST_ParseFlags_CONTAINS() {
        let flags = ParseFlags::ORDER_FIELDS | ParseFlags::ELIDE_BLANK_RECORDS;

        assert!(flags.contains(ParseFlags::ORDER_FIELDS));
        assert!(flags.contains(ParseFlags::ELIDE_BLANK_RECORDS));
        assert!(!flags.contains(ParseFlags::IGNORE_CASE_ON_LOOKUP));
    }


    #[test]
    fn TEST_ParseFlags_EMPTY() {
        assert_eq!(0, ParseFlags::empty().bits());
    }
}
