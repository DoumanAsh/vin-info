use core::fmt;

mod country;
mod manufacturer;

pub use country::map_wmi_to_country;
pub use manufacturer::map_wmi_to_manufacturer;

pub const WEIGHTS: [u32; 17] = [8, 7, 6, 5, 4, 3, 2, 10, 0, 9, 8, 7, 6, 5, 4, 3, 2];

pub const fn vin_char_weight(ch: char) -> u32 {
    match ch {
        '0' => 0,
        '1' | 'A' | 'J' => 1,
        '2' | 'B' | 'K' | 'S'  => 2,
        '3' | 'C' | 'L' | 'T' => 3,
        '4' | 'D' | 'M' | 'U' => 4,
        '5' | 'E' | 'N' | 'V' => 5,
        '6' | 'F' | 'W' => 6,
        '7' | 'G' | 'P' | 'X' => 7,
        '8' | 'H' | 'Y' => 8,
        '9' | 'R' | 'Z' => 9,
        _ => unreach!(),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
///Region
pub enum Region {
    ///Africa region
    Africa,
    ///Asia region
    Asia,
    ///Europe region
    Europe,
    ///North America region
    NorthAmerica,
    ///Oceania region
    Oceania,
    ///South America region
    SouthAmerica
}

impl Region {
    ///Returns region based on first character of WMI.
    ///
    ///If invalid, then return None
    pub const fn from_wmi_region(ch: u8) -> Option<Self> {
        match ch {
            b'A'..=b'H' => Some(Self::Africa),
            b'J'..=b'N' | b'P' | b'R' => Some(Self::Asia),
            b'S'..=b'Z' => Some(Self::Europe),
            b'1'..=b'5' => Some(Self::NorthAmerica),
            b'6'..=b'7' => Some(Self::Oceania),
            b'8'..=b'9' => Some(Self::SouthAmerica),
            _ => None,
        }
    }

    ///Gets textual representation
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Africa => "Africa",
            Self::Asia => "Asia",
            Self::Europe => "Europe",
            Self::NorthAmerica => "North America",
            Self::Oceania => "Oceania",
            Self::SouthAmerica => "South America",
        }
    }
}

impl fmt::Debug for Region {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), fmt)
    }
}

impl fmt::Display for Region {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
