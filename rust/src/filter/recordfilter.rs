use crate::filter::fieldfilter::FieldFilter;
use crate::layout::Layout;

// Char delimiter between field filters on the same condition
const FIELD_FILTER_DELIMITER: char = ';';

#[derive(Debug)]
pub struct RecordFilter {
    pub expr: Vec<FieldFilter>,
}

impl RecordFilter {
    /// Checks if all field names are found in the layout
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    /// use rbf::filter::recordfilter::RecordFilter;
    ///     
    /// let layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// let filters = RecordFilter::from("W10 = AA;N5 != 20");
    /// filters.check(layout);
    ///
    /// ```
    /// ```,should_panic
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    /// use rbf::filter::recordfilter::RecordFilter;
    ///     
    /// let layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// let filters = RecordFilter::from("FIELD1 = AA;N5 != 20");
    /// filters.check(layout);
    ///
    /// ```     
    pub fn check<T>(&self, layout: Layout<T>) {
        for expr in &self.expr {
            if !layout.contains_field(&expr.fname) {
                panic!("field name {} is not found in the layout!", expr.fname);
            }
        }
    }
}

/// Convenient conversion from a string ref.
///
/// # Examples
///
/// ```
/// use rbf::filter::recordfilter::RecordFilter;
///    
/// let filters = RecordFilter::from("FIELD1 = 10;FIELD2 != 20; FIELD3 ~ ^#");
/// assert_eq!(format!("{}", filters.expr[0]), "FIELD1=10");
/// assert_eq!(format!("{}", filters.expr[1]), "FIELD2!=20");
/// assert_eq!(format!("{}", filters.expr[2]), "FIELD3~^#");
/// ```
impl<'a> From<&'a str> for RecordFilter {
    fn from(op: &'a str) -> RecordFilter {
        let mut vec: Vec<FieldFilter> = Vec::new();

        // split according to delimiter
        for expr in op.split(FIELD_FILTER_DELIMITER) {
            vec.push(FieldFilter::from(expr));
        }

        RecordFilter { expr: vec }
    }
}
