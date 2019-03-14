use std::fmt;

use crate::types::datetime::{DateType, TimeType};
use crate::types::decimal::DecimalType;
use crate::types::int::SignedIntegerType;
use crate::types::string::StringType;
use crate::types::uint::UnsignedIntegerType;

#[macro_export]
macro_rules! get_name {
    ($sel:ident, $type_name:expr) => (
        fn get_name(&$sel) -> &'static str { $type_name }
    );
}

#[macro_export]
macro_rules! set_format {
    ($sel:ident, $fmt:ident) => (
        fn set_format(&mut $sel, $fmt: &str) { $sel.set_format($fmt); }
    );
}

#[macro_export]
macro_rules! get_format {
    ($sel:ident) => (
        fn get_format(&$sel) -> &str { &$sel.format }
    );
}

pub trait BaseType {
    //fn get_name(&self) -> String { self.get_type_name() }
    fn get_name(&self) -> &'static str;
    fn set_format(&mut self, fmt: &str);
    fn get_format(&self) -> &str;
    fn eq(&self, lhs: &str, rhs: &str) -> bool;
    fn lt(&self, lhs: &str, rhs: &str) -> bool;
    fn gt(&self, lhs: &str, rhs: &str) -> bool;
}

/// Convenient conversion from a string ref.
impl<'a> From<&'a str> for Box<BaseType> {
    fn from(original: &'a str) -> Box<BaseType> {
        match original {
            "string" => Box::new(StringType::new("")),
            "decimal" => Box::new(DecimalType::new("")),
            "int" => Box::new(SignedIntegerType::new("")),
            "uint" => Box::new(UnsignedIntegerType::new("")),
            "date" => Box::new(DateType::new("")),
            "time" => Box::new(TimeType::new("")),
            unknown_type @ _ => panic!("<{}> is not allowed as a field type", unknown_type),
        }
    }
}

impl fmt::Debug for Box<BaseType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {

    use crate::types::base::BaseType;

    #[test]
    fn comparison() {
        // test all types
        let target: Vec<&str> = "FOO;3.14;-100;100;20170101;120000".split(';').collect();
        let behind: Vec<&str> = "FOM;3.13;-101;99;20161231;115959".split(';').collect();
        let over: Vec<&str> = "FOP;3.15;-99;101;20170102;120001".split(';').collect();

        // string, etc
        let st = Box::<BaseType>::from("string");
        assert!(st.eq(target[0], target[0]));
        assert!(st.lt(behind[0], target[0]));
        assert!(st.gt(over[0], target[0]));

        let ft = Box::<BaseType>::from("decimal");
        assert!(ft.eq(target[1], target[1]));
        assert!(ft.lt(behind[1], target[1]));
        assert!(ft.gt(over[1], target[1]));

        let it = Box::<BaseType>::from("int");
        assert!(it.eq(target[2], target[2]));
        assert!(it.lt(behind[2], target[2]));
        assert!(it.gt(over[2], target[2]));

        let ut = Box::<BaseType>::from("uint");
        assert!(ut.eq(target[3], target[3]));
        assert!(ut.lt(behind[3], target[3]));
        assert!(ut.gt(over[3], target[3]));

        let mut dt = Box::<BaseType>::from("date");
        dt.set_format("%Y%m%d");
        assert!(dt.eq(target[4], target[4]));
        assert!(dt.lt(behind[4], target[4]));
        assert!(dt.gt(over[4], target[4]));

        let mut tt = Box::<BaseType>::from("time");
        tt.set_format("%H%M%S");
        assert!(tt.eq(target[5], target[5]));
        assert!(tt.lt(behind[5], target[5]));
        assert!(tt.gt(over[5], target[5]));
    }
}
