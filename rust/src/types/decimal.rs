use crate::types::base::BaseType;
use crate::types::compare::Compare;
use crate::types::root::RootType;

type Decimal = f64;
pub type DecimalType = RootType<Decimal>;

impl BaseType for DecimalType {
    // Returns name of the type
    get_name!(self, "decimal");

    /// Sets the format using the RootType method
    set_format!(self, fmt);

    /// Gets for format string.
    get_format!(self);

    fn eq(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<Decimal>::eq(lhs, rhs)
    }

    fn lt(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<Decimal>::lt(lhs, rhs)
    }

    fn gt(&self, lhs: &str, rhs: &str) -> bool {
        Compare::<Decimal>::gt(lhs, rhs)
    }
}
