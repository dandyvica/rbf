use crate::types::base::BaseType;
use crate::types::root::RootType;

pub type StringType = RootType<String>;

impl BaseType for StringType {
    // Returns name of the type
    get_name!(self, "string");

    /// Sets the format using the RootType method
    set_format!(self, fmt);

    /// Gets for format string.
    get_format!(self);

    // for strings, not need to call convert() first
    fn eq(&self, lhs: &str, rhs: &str) -> bool {
        lhs == rhs
    }

    // for strings, not need to call convert() first
    fn lt(&self, lhs: &str, rhs: &str) -> bool {
        lhs < rhs
    }

    // for strings, not need to call convert() first
    fn gt(&self, lhs: &str, rhs: &str) -> bool {
        lhs > rhs
    }
}
