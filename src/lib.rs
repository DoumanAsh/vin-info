//! VIN information library

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

use core::{fmt, str, slice, char};

#[cfg(debug_assertions)]
macro_rules! unreach {
    () => {
        unreachable!()
    }
}

#[cfg(not(debug_assertions))]
macro_rules! unreach {
    () => {
        unsafe {
            core::hint::unreachable_unchecked()
        }
    }
}

mod dicts;
pub use dicts::Region;

const VIN_LEN: usize = 17;

///Error parsing VIN
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VinError {
    ///VIN must be 17 characters long
    InvalidLen,
    ///VIN contains invalid character.
    InvalidChar(usize, char),
}

impl fmt::Display for VinError {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLen => fmt.write_str("VIN MUST be 17 characters long"),
            Self::InvalidChar(idx, ch) => fmt.write_fmt(format_args!("VIN contains invalid character '{ch}' at idx={idx}")),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
///Vehicle identifier number
pub struct Vin<'a>(&'a str);

impl<'a> Vin<'a> {
    ///Creates new instance with panic on invalid input.
    pub const fn new(vin: &'a str) -> Self {
        match Self::try_new(vin) {
            Ok(this) => this,
            Err(VinError::InvalidLen) => panic!("Invalid length of VIN"),
            Err(VinError::InvalidChar(_, _)) => panic!("VIN contains invalid character"),
        }
    }

    ///Creates new instance
    pub const fn try_new(vin: &'a str) -> Result<Self, VinError> {
        if vin.len() != VIN_LEN {
            return Err(VinError::InvalidLen);
        }

        let mut idx = 0;
        while idx < vin.len() {
            let ch = vin.as_bytes()[idx];
            match ch as char {
                //these are exceptions to letters
                'I' | 'O' | 'Q' => return Err(VinError::InvalidChar(idx, ch as char)),
                'A'..='Z' | '0'..='9' => idx += 1,
                ch => return Err(VinError::InvalidChar(idx, ch)),
            }
        }

        Ok(Self(vin))
    }

    ///Calculates checksum of the provided VIN
    pub const fn calculate_checksum(&self) -> u32 {
        let mut result = 0u32;
        let mut idx = 0;

        while idx < self.0.len() {
            let ch = self.0.as_bytes()[idx];
            //Example from wiki
            //https://en.wikipedia.org/wiki/Vehicle_identification_number#Worked_example
            result = result.wrapping_add(dicts::vin_char_weight(ch as char) * dicts::WEIGHTS[idx]);

            idx += 1;
        }

        result
    }

    ///Calculates checksum and transforms it into corresponding digit
    ///
    ///Note that this is only valid for North America/Asia VINs while eruopean VINs omit it completely and has no concept of checksum digit
    pub const fn calculate_checksum_digit(&self) -> char {
        let checksum = self.calculate_checksum();
        match checksum % 11 {
            10 => 'X',
            digit => match char::from_digit(digit, 10) {
                Some(ch) => ch,
                None => unreach!(),
            }
        }
    }

    #[inline(always)]
    ///Calculates checksum and compares it against VIN's checksum, returning true if equal
    pub const fn is_checksum_valid(&self) -> bool {
        let expected = self.calculate_checksum_digit();
        self.0.as_bytes()[8] as char == expected
    }

    const fn slice(&self, start: usize, len: usize) -> &str {
        unsafe {
            str::from_utf8_unchecked(
                slice::from_raw_parts(self.0.as_ptr().add(start), len)
            )
        }
    }

    #[inline(always)]
    ///Returns 3 letter World manufacturer identifier
    ///
    ///For some WMI may always ending with digit 9 (Check wikipedia for details)
    pub const fn wmi(&self) -> &str {
        self.slice(0, 3)
    }

    #[inline(always)]
    ///Returns vehicle description section
    ///
    ///Per ISO3779 it is characters from 4 to 9.
    ///But for North America/Asia character 9 usually acts as check digit.
    pub const fn vds(&self) -> &str {
        self.slice(3, 6)
    }

    #[inline(always)]
    ///Returns Vehicle identifier section (Characters 10 to 17)
    ///
    ///For North America/Asia it includes model year and plant code/manufacturer identifier
    ///Hence actual serial number starts from character 12, always numeric
    ///
    ///For Europe you can consider whole VIC as serial number.
    pub const fn vic(&self) -> &str {
        self.slice(9, 8)
    }

    #[inline(always)]
    ///Returns region manufacturer, if VIN is valid
    pub const fn manufacturer_region(&self) -> Option<Region> {
        Region::from_wmi_region(self.0.as_bytes()[0])
    }

    #[inline(always)]
    ///Returns manufacturer country, if VIN is valid and it is known value, otherwise 'Unknown'.
    pub const fn manufacturer_country(&self) -> &'static str {
        dicts::map_wmi_to_country(self.wmi())
    }

    #[inline(always)]
    ///Returns manufacturer name, if VIN is valid and it is known value, otherwise 'Unknown'.
    pub const fn manufacturer_name(&self) -> &'static str {
        dicts::map_wmi_to_manufacturer(self.wmi())
    }
}

impl fmt::Debug for Vin<'_> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.0, fmt)
    }
}

impl fmt::Display for Vin<'_> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0, fmt)
    }
}
