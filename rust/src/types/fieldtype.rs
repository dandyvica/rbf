//! Represents a basic type used by fields. Each field can be associated with a standard type,
//! which defines the type data it holds.
//!
//! 5 different types can be used, but it can be easily extended if desired:
//!
//!  * `string`
//!  * `integer`
//!  * `decimal`
//!  * `date`
//!  * `time`
//!
//! # Examples
//! ```rust
//! use rbf::types::fieldtype::FieldType;
//!
//! let ft = FieldType::new("I", "int");
//!
//! assert_eq!(&ft.id, "I");
//! assert_eq!(ft.type_as_string, "int");
//! ```

use regex::Regex;
use std::fmt;

use crate::types::base::BaseType;

pub struct FieldType {
    /// Nickname for the field type
    pub id: String,
    /// Name of the type as a string
    pub type_as_string: String,
    /// Base type (which is only limited to a set a values)
    pub base_type: Box<BaseType>,
    /// Optional pattern which describes field format
    pub pattern: Regex,
}

impl FieldType {
    /// Creates a new `FieldType` with an ID (a kind of nickname to refer to) and
    /// a type which should in the list: string, decimal, integer, date or time.
    ///
    /// # Arguments
    ///
    /// * `id` - nickname for the field type
    /// * `type_as_string`: base underlying type
    ///    
    pub fn new(id: &str, type_as_string: &str) -> FieldType {
        // first test arguments: non-sense to deal with empty data
        if id.is_empty() {
            panic!("cannot create a FieldType with empty id!");
        }

        // according to string type, create corresponding type
        FieldType {
            id: id.to_string(),
            type_as_string: type_as_string.to_string(),
            base_type: Box::<BaseType>::from(type_as_string),
            pattern: Regex::new("").unwrap(),
        }
    }

    /// Sets the regex pattern for the field type.
    ///
    /// # Arguments
    ///
    /// * `pattern` - string regex
    ///
    pub fn set_pattern(&mut self, pattern: &str) {
        self.pattern = Regex::new(pattern).unwrap();
    }
}

// implement display trait
impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id=<{}> type_as_string=<{}> pattern=<{}> format=<{}>",
            self.id,
            self.type_as_string,
            self.pattern,
            self.base_type.get_format()
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::types::fieldtype::FieldType;

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn unknown_fieldtype() {
        let ft = FieldType::new("C", "complex");
    }

    #[test]
    fn fieldtype_all() {
        let ft = FieldType::new("S", "string");
        assert_eq!(&ft.id, "S");
        assert_eq!(ft.type_as_string, "string");

        let ft = FieldType::new("N", "decimal");
        assert_eq!(&ft.id, "N");
        assert_eq!(ft.type_as_string, "decimal");

        let ft = FieldType::new("I", "int");
        assert_eq!(&ft.id, "I");
        assert_eq!(ft.type_as_string, "int");

        let ft = FieldType::new("U", "uint");
        assert_eq!(&ft.id, "U");
        assert_eq!(ft.type_as_string, "uint");

        /*        let ft = FieldType::new("D", "date");
        assert_eq!(&ft.id, "D");
        assert_eq!(ft.base_type, BaseDataType::Date{ date_format: "%D%m%s".to_string() });

        let ft = FieldType::new("T", "time");
        assert_eq!(&ft.id, "T");
        assert_eq!(ft.base_type, BaseDataType::Time{ time_format: "%H%M%S".to_string() });  */
    }
}
