//! Represents a data field by its name, description, type and length.
//!
//! This struct should be used with its companion struct [Record](../record/struct.Record.html). If a record can
//! be mapped to a line of text within a file, then a field is a substring from
//! that line, with a fixed length.
//!
//! Each field is holding the substring in the **value()** and **raw_value()** properties.
//!
//! # Examples
//! ```rust
//! use std::rc::Rc;
//! use rbf::types::fieldtype::FieldType;
//! use rbf::field::Field;
//!
//! let ft = Rc::new(FieldType::new("I", "int"));
//! let mut f1 = Field::from_length("F1", "Description for field 1", &ft, 10);
//!
//! assert_eq!(&f1.name, "F1");
//! assert_eq!(&f1.description, "Description for field 1");
//! assert_eq!(f1.length, 10);
//!
//! f1.set_value("  XX  ");
//! assert_eq!(f1.value(), "XX");
//!
//! let other_f1 = f1.clone();
//! assert_eq!(other_f1.value(), "XX");
//! ```

use std::cmp::max;
use std::fmt;
use std::rc::Rc;

use crate::errmsg::*;
use crate::error_msg;
use crate::filter::fieldfilter::{FieldFilter, FieldFilterOp};
use crate::types::fieldtype::FieldType;

/// Holds the way a **Field** is defined: by giving its length or its offsets
#[derive(Debug, Clone)]
pub enum FieldCreationType {
    ByLength,
    ByOffset,
}

#[derive(Debug, Clone)]
pub struct Field {
    /// field name
    pub name: String,
    /// field description
    pub description: String,
    /// field length in chars
    pub length: usize,
    /// field type of this field, in chars (but not in bytes, because of UTF-8 strings)
    pub ftype: Rc<FieldType>,
    /// field value, copied as-is
    pub raw_value: String,
    /// blank-stripped field value
    pub str_value: String,
    /// offset in chars of this field within its parent record
    pub offset_from_origin: usize,
    /// index of this field within its record
    pub index: usize,
    /// first position (in chars) from the beginning of the record
    pub lower_offset: usize,
    /// last position (in chars) of the field within its record
    pub upper_offset: usize,
    /// in case of a record having the same field name several times, this tracks down the field number
    pub multiplicity: usize,
    /// for display purpose (= maximum of field description and length)
    pub cell_size: usize,
    /// holds the way a Field is created: by length or by offset
    pub creation_type: FieldCreationType,
    /// unique name associated to the field: in case of field duplication, this name is unique
    pub id: String,
}

impl Field {
    /// Creates a new field with length.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the field
    /// * `description`: description of the field
    /// * `FieldType` fieldtype: format of the field (type of data found in the field)
    /// * `length`: number of chars of the field
    ///
    /// # Panics
    /// If `name` is empty or `length` is 0
    ///
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;  
    ///  
    /// let ft = Rc::new(FieldType::new("I", "int"));
    /// let mut f1 = Field::from_length("F1", "Description for field 1", &ft, 10);     
    ///
    /// assert_eq!(&f1.name, "F1");
    /// assert_eq!(&f1.description, "Description for field 1");    
    /// assert_eq!(f1.length, 10);    
    /// ```    
    ///
    /// ```should_panic
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;      
    /// let ft = Rc::new(FieldType::new("I", "int"));
    ///
    /// let f1 = Field::from_length("F1", "Description for field 1", &ft, 0);
    /// let f2 = Field::from_length("", "Description for field 1", &ft, 10);
    /// ```            
    pub fn from_length(
        name: &str,
        description: &str,
        ftype: &Rc<FieldType>,
        length: usize,
    ) -> Field {
        // test arguments: non-sense to deal with empty data
        if name.is_empty() {
            panic!(MSG0110);
        }
        if length == 0 {
            panic!(MSG0111);
        }

        // initialize all relevant members
        Field {
            name: name.to_string(),
            description: description.to_string(),
            length: length,
            ftype: ftype.clone(),
            raw_value: String::new(),
            str_value: String::new(),
            offset_from_origin: 0,
            index: 0,
            lower_offset: 0,
            upper_offset: 0,
            multiplicity: 0,
            cell_size: max(length, name.len()),
            creation_type: FieldCreationType::ByLength,
            id: String::new(),
        }
    }

    /// Creates a new field with lower & upper bounds.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the field
    /// * `description`: description of the field
    /// * `FieldType` fieldtype: format of the field (type of data found in the field)
    /// * `lower_offset`: lower bound of the field in the record
    /// * `upper_offset`: upper bound of the field in the record
    ///
    /// # Panics
    /// If `name` is empty or `lower_offset > upper_offset`
    ///
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;  
    ///  
    /// let ft = Rc::new(FieldType::new("I", "int"));
    /// let mut f1 = Field::from_length("F1", "Description for field 1", &ft, 10);     
    ///
    /// assert_eq!(&f1.name, "F1");
    /// assert_eq!(&f1.description, "Description for field 1");    
    /// assert_eq!(f1.length, 10);
    /// ```  
    ///
    /// ```should_panic
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;      
    /// let ft = Rc::new(FieldType::new("I", "int"));
    ///
    /// let f3 = Field::from_offset("F1", "Description for field 1", &ft, 10, 5);
    /// let f2 = Field::from_offset("", "Description for field 1", &ft, 5, 10);
    /// ```        
    pub fn from_offset(
        name: &str,
        description: &str,
        ftype: &Rc<FieldType>,
        lower_offset: usize,
        upper_offset: usize,
    ) -> Field {
        // test arguments: non-sense to deal with empty data
        if name.is_empty() {
            panic!(error_msg!(MSG0110));
        }
        // sanity check
        if lower_offset > upper_offset {
            panic!(
                "error creating field {}: lower offset {} > upper offset {}!",
                name, lower_offset, upper_offset
            );
            //panic!(error_msg!(MSG0112, name, lower_offset, upper_offset));
        }

        // calculate length & initialize all relevant members
        let length = upper_offset - lower_offset + 1;

        Field {
            name: name.to_string(),
            description: description.to_string(),
            length: length,
            ftype: ftype.clone(),
            raw_value: String::new(),
            str_value: String::new(),
            offset_from_origin: 0,
            index: 0,
            lower_offset: lower_offset - 1, // internally kept at origin 0
            upper_offset: upper_offset - 1, // internally kept at origin 0
            multiplicity: 0,
            cell_size: max(length, name.len()),
            creation_type: FieldCreationType::ByOffset,
            id: String::new(),
        }
    }

    /// Sets the value which is blank-stripped and also kept asis in the **raw_value** struct field.
    ///
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;  
    ///  
    /// let ft = Rc::new(FieldType::new("S", "string"));
    /// let mut f = Field::from_length("F1", "Description for field 1", &ft, 10);     
    ///
    /// // utf-8
    /// f.set_value("  αβ  ");    
    /// assert_eq!(f.raw_value, "  αβ  ");
    /// assert_eq!(f.str_value, "αβ");    
    ///
    /// // ascii
    /// f.set_value("  XX  ");    
    /// assert_eq!(f.raw_value, "  XX  ");  
    /// assert_eq!(f.str_value, "XX");         
    /// ```      
    pub fn set_value(&mut self, val: &str) {
        self.str_value = String::from(val.trim());
        self.raw_value = String::from(val);
    }

    /// Returns the field value.
    ///
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;  
    ///  
    /// let ft = Rc::new(FieldType::new("S", "string"));
    /// let mut f = Field::from_length("F1", "Description for field 1", &ft, 10);     
    ///
    /// // utf-8
    /// f.set_value("  αβ  ");    
    /// assert_eq!(f.value(), "αβ");    
    ///
    /// // ascii
    /// f.set_value("  XX  ");    
    /// assert_eq!(f.value(), "XX");         
    /// ```      
    pub fn value(&self) -> &String {
        &self.str_value
    }

    /// Returns the total number of chars in the fields.
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;  
    ///  
    /// let ft = Rc::new(FieldType::new("I", "int"));
    /// let mut f = Field::from_length("F1", "Description for field 1", &ft, 10);     
    ///
    /// assert_eq!(f.len(), 10);
    /// ```     
    pub fn len(&self) -> usize {
        self.length
    }

    /// Verifies if the field value is matching the field type pattern.
    ///
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;  
    ///  
    /// let mut ft = FieldType::new("I", "int");
    /// ft.set_pattern("\\d+");
    ///
    /// let mut f = Field::from_offset("F1", "Description for field 1", &Rc::new(ft), 5, 10);
    /// f.set_value("123");  
    /// assert!(f.is_pattern_matched());
    ///
    /// f.set_value("ABC");  
    /// assert!(!f.is_pattern_matched());  
    /// ```         
    pub fn is_pattern_matched(&self) -> bool {
        self.ftype.pattern.is_match(&self.raw_value)
    }

    /// Checks if the field value matches the field filter
    ///
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    ///
    /// use rbf::types::fieldtype::FieldType;
    /// use rbf::field::Field;
    /// use rbf::filter::fieldfilter::FieldFilter;     
    ///  
    /// let expr = FieldFilter::from("FIELD1 ~ ^AA");
    /// let ft = Rc::new(FieldType::new("S", "string"));
    ///
    /// let mut f = Field::from_length("FIELD1", "Description for field 1", &ft, 10);     
    /// f.set_value("AAAAAA");
    /// assert!(f.is_filter_matched(&expr));
    ///
    /// f.set_value("ABAAAA");
    /// assert!(!f.is_filter_matched(&expr));     
    /// ```       
    pub fn is_filter_matched(&self, filter: &FieldFilter) -> bool {
        let result = match filter.op {
            FieldFilterOp::OpEqual => self
                .ftype
                .base_type
                .eq(self.value(), filter.freg_or_value.as_str()),
            FieldFilterOp::OpNotEqual => !self
                .ftype
                .base_type
                .eq(self.value(), filter.freg_or_value.as_str()),
            FieldFilterOp::OpSimilar => filter.freg_or_value.is_match(&self.value()),
            FieldFilterOp::OpNotSimilar => !filter.freg_or_value.is_match(&self.value()),
            FieldFilterOp::OpLessThan => self
                .ftype
                .base_type
                .lt(self.value(), filter.freg_or_value.as_str()),
            FieldFilterOp::OpGreaterThan => self
                .ftype
                .base_type
                .gt(self.value(), filter.freg_or_value.as_str()),
        };
        result
    }
}

/// Implements **Display** trait: just print out field name and field value
///
/// # Examples
/// ```
/// use std::rc::Rc;
///
/// use rbf::types::fieldtype::FieldType;
/// use rbf::field::Field;  
///  
/// let ft = Rc::new(FieldType::new("S", "string"));
/// let mut f = Field::from_length("F1", "Description for field 1", &ft, 10);     
///
/// f.set_value("  XX  ");    
/// assert_eq!(format!("{}", f), "F1='XX'");      
/// ```  
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}='{}'", self.name, self.value())
    }
}
