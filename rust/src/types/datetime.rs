use chrono::prelude::*;
use std::error::Error;

use crate::types::base::BaseType;
use crate::types::root::RootType;

fn to_date(value: &str, fmt: &str) -> NaiveDate {
    let converted = match NaiveDate::parse_from_str(value, fmt) {
        Ok(v) => v,
        Err(e) => panic!(
            "unable to convert string value {}, error={}",
            value,
            e.description()
        ),
    };
    converted
}

fn to_time(value: &str, fmt: &str) -> NaiveTime {
    let converted = match NaiveTime::parse_from_str(value, fmt) {
        Ok(v) => v,
        Err(e) => panic!(
            "unable to convert string value {}, error={}",
            value,
            e.description()
        ),
    };
    converted
}

pub type DateType = RootType<NaiveDate>;
pub type TimeType = RootType<NaiveTime>;

impl BaseType for DateType {
    // Returns name of the type
    get_name!(self, "date");

    /// Sets the format using the RootType method
    set_format!(self, fmt);

    /// Gets for format string.
    get_format!(self);

    // for strings, not need to call convert() first
    fn eq(&self, lhs: &str, rhs: &str) -> bool {
        to_date(lhs, &self.format) == to_date(rhs, &self.format)
    }

    // for strings, not need to call convert() first
    fn lt(&self, lhs: &str, rhs: &str) -> bool {
        to_date(lhs, &self.format) < to_date(rhs, &self.format)
    }

    // for strings, not need to call convert() first
    fn gt(&self, lhs: &str, rhs: &str) -> bool {
        to_date(lhs, &self.format) > to_date(rhs, &self.format)
    }
}

impl BaseType for TimeType {
    // Returns name of the type
    get_name!(self, "time");

    /// Sets the format using the RootType method
    set_format!(self, fmt);

    /// Gets for format string.
    get_format!(self);

    // for strings, not need to call convert() first
    fn eq(&self, lhs: &str, rhs: &str) -> bool {
        to_time(lhs, &self.format) == to_time(rhs, &self.format)
    }

    // for strings, not need to call convert() first
    fn lt(&self, lhs: &str, rhs: &str) -> bool {
        to_time(lhs, &self.format) < to_time(rhs, &self.format)
    }

    // for strings, not need to call convert() first
    fn gt(&self, lhs: &str, rhs: &str) -> bool {
        to_time(lhs, &self.format) > to_time(rhs, &self.format)
    }
}
