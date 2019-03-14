use regex::Regex;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum FieldFilterOp {
    OpEqual,
    OpNotEqual,
    OpSimilar,
    OpNotSimilar,
    OpLessThan,
    OpGreaterThan,
}

/// Convenient conversion from a string ref.
///
/// # Examples
///
/// ```
/// use rbf::filter::fieldfilter::FieldFilterOp;
///    
/// let ffop = FieldFilterOp::from("<");
/// ```
///
/// ```should_panic
/// use rbf::filter::fieldfilter::FieldFilterOp;
///    
/// let ffop = FieldFilterOp::from("#");
/// ```
impl<'a> From<&'a str> for FieldFilterOp {
    fn from(op: &'a str) -> FieldFilterOp {
        match op {
            "=" => FieldFilterOp::OpEqual,
            "!=" => FieldFilterOp::OpNotEqual,
            "~" => FieldFilterOp::OpSimilar,
            "!~" => FieldFilterOp::OpNotSimilar,
            "<" => FieldFilterOp::OpLessThan,
            ">" => FieldFilterOp::OpGreaterThan,
            unknown_op @ _ => panic!(
                "<{}> is not allowed as a field expression operator",
                unknown_op
            ),
        }
    }
}

#[derive(Debug)]
pub struct FieldFilter {
    // field name
    pub fname: String,
    // operator as a string
    pub op_string: String,
    // operator as an enum variant
    pub op: FieldFilterOp,
    // regex value to match
    pub freg_or_value: Regex,
}

/// Creates a new field filter
///
/// # Examples
///
/// ```
/// use rbf::filter::fieldfilter::{FieldFilterOp,FieldFilter};
///    
/// let expr = FieldFilter::new("  FIELD1  ", " =  ", " FOO  ");
/// assert_eq!(expr.fname, "FIELD1");
/// assert_eq!(expr.op, FieldFilterOp::OpEqual);
/// assert_eq!(expr.freg_or_value.as_str(), "FOO");
/// ```
impl FieldFilter {
    pub fn new(fname: &str, op_str: &str, fvalue: &str) -> FieldFilter {
        FieldFilter {
            fname: fname.trim().to_string(),
            op_string: op_str.trim().to_owned(),
            op: FieldFilterOp::from(op_str.trim()),
            freg_or_value: Regex::new(fvalue.trim()).unwrap(),
        }
    }
}

/// Convenient conversion from a string ref.
///
/// # Examples
///
/// ```
/// use rbf::filter::fieldfilter::{FieldFilterOp, FieldFilter};
///    
/// let mut expr = FieldFilter::from("  FIELD1 = FOO   ");
/// assert_eq!(expr.fname, "FIELD1");
/// assert_eq!(expr.op, FieldFilterOp::OpEqual);
/// assert_eq!(expr.freg_or_value.as_str(), "FOO");
///
/// expr = FieldFilter::from("FIELD1 != FOO");
/// assert_eq!(expr.fname, "FIELD1");
/// assert_eq!(expr.op, FieldFilterOp::OpNotEqual);
/// assert_eq!(expr.freg_or_value.as_str(), "FOO");
///
/// expr = FieldFilter::from("FIELD1 ~ ^FOO");
/// assert_eq!(expr.fname, "FIELD1");
/// assert_eq!(expr.op, FieldFilterOp::OpSimilar);
/// assert_eq!(expr.freg_or_value.as_str(), "^FOO");
///
/// expr = FieldFilter::from("FIELD1 !~ ^FOO");
/// assert_eq!(expr.fname, "FIELD1");
/// assert_eq!(expr.op, FieldFilterOp::OpNotSimilar);
/// assert_eq!(expr.freg_or_value.as_str(), "^FOO");
///
/// expr = FieldFilter::from("FIELD1 < 10");
/// assert_eq!(expr.fname, "FIELD1");
/// assert_eq!(expr.op, FieldFilterOp::OpLessThan);
/// assert_eq!(expr.freg_or_value.as_str(), "10");
///
/// expr = FieldFilter::from("FIELD1 > 10");
/// assert_eq!(expr.fname, "FIELD1");
/// assert_eq!(expr.op, FieldFilterOp::OpGreaterThan);
/// assert_eq!(expr.freg_or_value.as_str(), "10");
/// ```
impl<'a> From<&'a str> for FieldFilter {
    fn from(expr: &'a str) -> FieldFilter {
        // regex used to split expression
        let expr_reg = Regex::new(r"(?P<field>\w+)\s+(?P<op>=|!=|~|!~|<|>)\s+(?P<re>.+)$").unwrap();

        // split according to delimiter
        let caps = match expr_reg.captures(expr) {
            Some(e) => e,
            None => panic!("unable to find a suitable operator for filter \"{}\"", expr),
        };

        FieldFilter::new(&caps["field"], &caps["op"], &caps["re"])
    }
}

/// Displays field filter expression.
///
/// # Examples
///
/// ```
/// use rbf::filter::fieldfilter::FieldFilter;
///    
/// let expr = FieldFilter::new("  FIELD1  ", " !=  ", " FOO  ");
/// assert_eq!(format!("{}", expr), "FIELD1!=FOO");
/// ```
impl fmt::Display for FieldFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.fname,
            self.op_string,
            self.freg_or_value.as_str()
        )
    }
}
