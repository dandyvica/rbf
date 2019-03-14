use crate::types::base::BaseType;
use crate::types::compare::Compare;
use crate::types::root::RootType;

type SignedInteger = i64;
pub type SignedIntegerType = RootType<SignedInteger>;

impl BaseType for SignedIntegerType {
    // Returns name of the type
    get_name!(self, "int");

    /// Sets the format using the RootType method
    set_format!(self, fmt);

    /// Gets for format string.
    get_format!(self);

    fn eq(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<SignedInteger>::eq(lhs, rhs)
    }

    fn lt(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<SignedInteger>::lt(lhs, rhs)
    }

    fn gt(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<SignedInteger>::gt(lhs, rhs)
    }
}
