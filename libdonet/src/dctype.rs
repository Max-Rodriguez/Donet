/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez

    Donet is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License,
    as published by the Free Software Foundation, either version 3
    of the License, or (at your option) any later version.

    Donet is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with Donet. If not, see <https://www.gnu.org/licenses/>.
*/

//! Represents all data types supported by the DC language
//! and developer-defined type alias definitions.

use crate::globals::DgSizeTag;
use crate::hashgen::DCHashGenerator;
use strum_macros::EnumIs;

/// The DCTypeEnum variants have assigned u8 values
/// to keep compatibility with Astron's DC hash inputs.
#[repr(u8)] // 8-bit alignment, unsigned
#[derive(Debug, Clone, PartialEq, Eq)]
#[rustfmt::skip]
pub enum DCTypeEnum {
    // Numeric Types
    TInt8 = 0, TInt16 = 1, TInt32 = 2, TInt64 = 3,
    TUInt8 = 4, TChar = 8, TUInt16 = 5, TUInt32 = 6, TUInt64 = 7,
    TFloat32 = 9, TFloat64 = 10,

    // Sized Data Types (Array Types)
    TString = 11, // a string with a fixed byte length
    TVarString = 12, // a string with a variable byte length
    TBlob = 13, TVarBlob = 14,
    TBlob32 = 19, TVarBlob32 = 20,
    TArray = 15, TVarArray = 16,

    // Complex DC Types
    TStruct = 17, TMethod = 18,
    TInvalid = 21,
}

impl std::fmt::Display for DCTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TInt8 => write!(f, "int8"),
            Self::TInt16 => write!(f, "int16"),
            Self::TInt32 => write!(f, "int32"),
            Self::TInt64 => write!(f, "int64"),
            Self::TUInt8 => write!(f, "uint8"),
            Self::TChar => write!(f, "char"),
            Self::TUInt16 => write!(f, "uint16"),
            Self::TUInt32 => write!(f, "uint32"),
            Self::TUInt64 => write!(f, "uint64"),
            Self::TFloat32 => write!(f, "float32"),
            Self::TFloat64 => write!(f, "float64"),
            Self::TString => write!(f, "string"),
            Self::TVarString => write!(f, "var string"),
            Self::TBlob => write!(f, "blob"),
            Self::TVarBlob => write!(f, "var blob"),
            Self::TBlob32 => write!(f, "blob32"),
            Self::TVarBlob32 => write!(f, "var blob32"),
            Self::TArray => write!(f, "array"),
            Self::TVarArray => write!(f, "var array"),
            Self::TStruct => write!(f, "struct"),
            Self::TMethod => write!(f, "method"),
            Self::TInvalid => write!(f, "invalid"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DCTypeDefinition {
    alias: Option<String>,
    pub data_type: DCTypeEnum,
    pub size: DgSizeTag,
}

impl Default for DCTypeDefinition {
    fn default() -> Self {
        Self {
            alias: None,
            data_type: DCTypeEnum::TInvalid,
            size: 0_u16,
        }
    }
}

impl DCTypeDefinition {
    /// Creates a new empty DCTypeDefinition struct.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Creates a new DCTypeDefinition struct with a DC type set.
    pub fn new_with_type(dt: DCTypeEnum) -> Self {
        Self {
            alias: None,
            data_type: dt,
            size: 0_u16,
        }
    }

    /// Accumulates the properties of this DC element into the file hash.
    pub fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        hashgen.add_int(i32::from(self.data_type.clone() as u8));

        if self.alias.is_some() {
            hashgen.add_string(self.alias.clone().unwrap())
        }
    }

    pub fn get_dc_type(&self) -> DCTypeEnum {
        self.data_type.clone()
    }

    #[inline(always)]
    pub fn is_variable_length(&self) -> bool {
        self.size == 0_u16
    }

    #[inline(always)]
    pub fn get_size(&self) -> DgSizeTag {
        self.size
    }

    #[inline(always)]
    pub fn has_alias(&self) -> bool {
        self.alias.is_some()
    }

    pub fn get_alias(&self) -> Result<String, ()> {
        if self.alias.is_some() {
            Ok(self.alias.clone().unwrap())
        } else {
            Err(())
        }
    }

    pub fn set_alias(&mut self, alias: String) {
        self.alias = Some(alias);
    }
}

impl std::fmt::Display for DCTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "typedef ")?;
        self.data_type.fmt(f)?;
        if self.has_alias() {
            write!(f, " ")?;
            self.alias.clone().unwrap().fmt(f)?;
        }
        write!(f, ";")?;
        writeln!(f)
    }
}

// ---------- DC Number ---------- //

#[rustfmt::skip]
#[derive(Clone, PartialEq, EnumIs)]
pub enum DCNumberType {
    None = 0, Int, UInt, Float,
}

#[repr(C)]
#[derive(Copy, Clone)] // required for unwrapping when in an option type
pub union DCNumberValueUnion {
    pub integer: i64,
    pub unsigned_integer: u64,
    pub floating_point: f64,
}

#[derive(Clone)]
pub struct DCNumber {
    pub number_type: DCNumberType,
    pub value: DCNumberValueUnion,
}

// We have to manually implement the 'PartialEq' trait
// due to the usage of a union data type.
impl PartialEq for DCNumber {
    fn eq(&self, rhs: &Self) -> bool {
        self.number_type == rhs.number_type
    }
}

impl Default for DCNumber {
    fn default() -> Self {
        Self {
            number_type: DCNumberType::None,
            value: DCNumberValueUnion { integer: 0_i64 },
        }
    }
}

impl DCNumber {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_integer(num: i64) -> Self {
        Self {
            number_type: DCNumberType::Int,
            value: DCNumberValueUnion { integer: num },
        }
    }

    pub fn new_unsigned_integer(num: u64) -> Self {
        Self {
            number_type: DCNumberType::UInt,
            value: DCNumberValueUnion {
                unsigned_integer: num,
            },
        }
    }

    pub fn new_floating_point(num: f64) -> Self {
        Self {
            number_type: DCNumberType::Float,
            value: DCNumberValueUnion { floating_point: num },
        }
    }
}
