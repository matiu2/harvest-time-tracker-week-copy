//! Handy traits to convert dates

use chrono::NaiveDate;

/// Converts a string date to a NaiveDate
pub trait FromHarvestDate {
    fn from_harvest_date(self) -> NaiveDate;
}

/// Converts a NaiveDate a string in the format that harvest api requires
pub trait ToHarvestDate {
    fn to_harvest_date(self) -> String;
}

impl<'a> FromHarvestDate for &'a str {
    fn from_harvest_date(self) -> NaiveDate {
        NaiveDate::parse_from_str(self, "%Y-%m-%d").unwrap()
    }
}

impl ToHarvestDate for NaiveDate {
    fn to_harvest_date(self) -> String {
        self.format("%Y-%m-%d").to_string()
    }
}
