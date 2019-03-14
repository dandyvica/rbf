use crate::types::base::BaseType;
use crate::types::compare::Compare;
use crate::types::root::RootType;

type UnsignedInteger = u64;
pub type UnsignedIntegerType = RootType<UnsignedInteger>;

impl BaseType for UnsignedIntegerType {
    // Returns name of the type
    get_name!(self, "uint");

    /// Sets the format using the RootType method
    set_format!(self, fmt);

    /// Gets for format string.
    get_format!(self);

    fn eq(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<UnsignedInteger>::eq(lhs, rhs)
    }

    fn lt(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<UnsignedInteger>::lt(lhs, rhs)
    }

    fn gt(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<UnsignedInteger>::gt(lhs, rhs)
    }
}
