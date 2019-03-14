//! Represents a record by its name, description and length. Each record contains a list of contiguous fields
//! which hold values when reading a record-based file.
//!
//! # Examples
//! ```rust
//! use std::rc::Rc;
//!
//! use rbf::types::fieldtype::FieldType;
//! use rbf::field::Field;
//! use rbf::record::{ReadMode, UTF8Mode, Record};
//!
//! let ft1 = Rc::new(FieldType::new("I", "int"));
//!
//! let f1 = Field::from_length("FIELD1", "Description for field 1", &ft1, 10);
//! let f2 = Field::from_length("FIELD2", "Description for field 2", &ft1, 10);
//! let f3 = Field::from_length("FIELD3", "Description for field 3", &ft1, 20);
//! let f4 = Field::from_length("FIELD2", "Description for field 2", &ft1, 10);
//!
//! let mut rec = Record::<UTF8Mode>::new("RECORD1", "Description for record 1", 20);
//!
//! rec.push(f1);
//! rec.push(f2);
//! rec.push(f3);
//! rec.push(f4);
//!
//! assert_eq!(rec.calculated_length, 50);
//! assert_eq!(rec.count(), 4);
//!
//! let s = "FIELD1".to_string();
//!
//! assert!(rec.contains_field(&s));
//! assert_eq!(rec.contains_field("FOO"), false);
//!
//! assert!(rec.get("FIELD1").is_some());
//! assert!(rec.get("FOO").is_none());
//!
//! let s2 = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
//! rec.set_value(&s2);
//! assert_eq!(rec[0].value(), "AAAAAAAAAA");
//! assert_eq!(rec[1].value(), "BBBBBBBBBB");
//! assert_eq!(rec[2].value(), "CCCCCCCCCCCCCCCCCCCC");
//! assert_eq!(rec[3].value(), "DDDDDDDDDD");
//!
//!
//! let s3 = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDEEEEEEEEEEEEEEEE";
//! rec.set_value(&s3);
//! assert_eq!(rec[0].value(), "AAAAAAAAAA");
//! assert_eq!(rec[1].value(), "BBBBBBBBBB");
//! assert_eq!(rec[2].value(), "CCCCCCCCCCCCCCCCCCCC");
//! assert_eq!(rec[3].value(), "DDDDDDDDDD");
//!
//!
//! let s4 = "AAAAAAAAAA";
//! rec.set_value(&s4);
//! assert_eq!(rec[0].value(), "AAAAAAAAAA");
//! assert_eq!(rec[1].raw_value, "          ");
//! assert_eq!(rec[2].raw_value, "                    ");
//! assert_eq!(rec[3].raw_value, "          ");
//! assert_eq!(rec[1].value(), "");
//! assert_eq!(rec[2].value(), "");
//! assert_eq!(rec[3].value(), "");
//!
//!
//! let s5 = "ααααααααααββββββββββγγγγγγγγγγγγγγγγγγγγδδδδδδδδδδ";
//! rec.set_value(&s5);
//! assert_eq!(rec[0].value(), "αααααααααα");
//! assert_eq!(rec[1].value(), "ββββββββββ");
//! assert_eq!(rec[2].value(), "γγγγγγγγγγγγγγγγγγγγ");
//! assert_eq!(rec[3].value(), "δδδδδδδδδδ");
//! ```

use std::borrow::Cow;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use crate::field::{Field, FieldCreationType};
use crate::filter::recordfilter::RecordFilter;

/// This allows to define a way to read either pure Ascii data or UTF-8 data. Because the way
/// of slicing is not the same, it's much more efficient using Ascii.
pub struct AsciiMode;
pub struct UTF8Mode;

/// This trait will be implemented by readers
pub trait ReadMode {
    fn set_value(&mut self, value: &str);
}

/// Implement Ascii read mode
impl ReadMode for Record<AsciiMode> {
    /// Sets the record value (which is equivalent to setting all fields).
    ///
    /// # Examples
    /// ```
    /// use rbf::vector_of;
    /// use rbf::record::ReadMode;
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();  
    ///
    /// let s = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
    /// rec.set_value(&s);
    ///
    /// assert_eq!(rec[0].value(), "AAAAAAAAAA");
    /// assert_eq!(rec[1].value(), "BBBBBBBBBB");
    /// assert_eq!(rec[2].value(), "CCCCCCCCCCCCCCCCCCCC");
    /// assert_eq!(rec[3].value(), "DDDDDDDDDD");
    /// assert_eq!(
    ///     vector_of!(rec, raw_value),
    ///     vec![
    ///         "AAAAAAAAAA",
    ///         "BBBBBBBBBB",
    ///         "CCCCCCCCCCCCCCCCCCCC",
    ///         "DDDDDDDDDD"
    ///     ]
    /// );    
    ///
    /// ```       
    fn set_value(&mut self, value: &str) {
        let s = self.adjust_value(value);

        // setting record value is setting value for all fields/records composing the record
        for f in &mut self.flist {
            let r = f.lower_offset..f.upper_offset + 1;
            f.set_value(&s[r]);
        }
    }
}

/// Implement UTF-8 read mode
impl ReadMode for Record<UTF8Mode> {
    /// Sets the record value (which is equivalent to setting all fields).
    ///
    /// # Examples
    /// ```
    /// use rbf::vector_of;
    /// use rbf::record::ReadMode;
    /// use rbf::record::{UTF8Mode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<UTF8Mode>();  
    ///
    /// let s = "ααααααααααββββββββββγγγγγγγγγγγγγγγγγγγγδδδδδδδδδδ";
    /// rec.set_value(&s);
    ///    
    /// assert_eq!(rec[0].value(), "αααααααααα");
    /// assert_eq!(rec[1].value(), "ββββββββββ");
    /// assert_eq!(rec[2].value(), "γγγγγγγγγγγγγγγγγγγγ");
    /// assert_eq!(rec[3].value(), "δδδδδδδδδδ");
    /// assert_eq!(
    ///     vector_of!(rec, raw_value),
    ///     vec![
    ///         "αααααααααα",
    ///         "ββββββββββ",
    ///         "γγγγγγγγγγγγγγγγγγγγ",
    ///         "δδδδδδδδδδ"
    ///     ]
    /// );    
    ///
    /// ```     
    fn set_value(&mut self, value: &str) {
        let s = self.adjust_value(value);

        // this is made for UTF-8 strings
        for f in &mut self.flist {
            let fvalue: String = s.chars().skip(f.lower_offset).take(f.length).collect();
            f.set_value(&fvalue);
        }
    }
}

/// Macro which builds a vector of Record data fields.
#[macro_export]
macro_rules! vector_of {
    ($rec: ident, $field: ident) => {{
        let v: Vec<_> = $rec.flist.iter().map(|f| f.$field.clone()).collect();
        v
    }};
}

#[derive(Clone)]
pub struct Record<T> {
    /// Record name
    pub name: String,
    /// Record description
    pub description: String,
    /// Wihtin a layout, records might have different lengths. In that case, this holds individual record length
    pub declared_length: usize,
    /// List of fields composing the record
    pub flist: Vec<Field>,
    /// Sum of all field lengths
    pub calculated_length: usize,
    /// Reader mode struct, just a place holder
    pub reader_mode: PhantomData<T>,
}

impl<T> Record<T> {
    /// Creates a new Record.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the record
    /// * `description`: description of the record
    /// * `length`: number of chars of the record
    ///
    /// # Panics
    /// If `name` is empty
    ///
    /// # Examples
    /// ```should_panic
    /// use rbf::record::{AsciiMode, Record};      
    /// let mut rec = Record::<AsciiMode>::new("", "Description for record 1", 20);
    /// ```     
    pub fn new(name: &str, description: &str, length: usize) -> Record<T> {
        // first test arguments: non-sense to deal with empty name
        // but description could be empty.
        if name.is_empty() {
            panic!("cannot create a record with an empty name!");
        }

        // initialize all relevant members
        Record {
            name: name.to_string(),
            description: description.to_string(),
            declared_length: length,
            flist: Vec::new(),
            calculated_length: 0,
            reader_mode: PhantomData,
        }
    }

    /// Adds a Field structure to the end of the record.
    ///
    /// # Examples
    /// ```
    /// use rbf::vector_of;
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();  
    ///  
    /// assert_eq!(rec.calculated_length, 50);
    /// assert_eq!(rec.count(), 4);
    ///
    /// assert_eq!(vector_of!(rec, name), vec!["FIELD1", "FIELD2", "FIELD3", "FIELD2"]);  
    /// assert_eq!(vector_of!(rec, description), vec!["Description for field 1", "Description for field 2", "Description for field 3", "Description for field 2"]);  
    /// assert_eq!(vector_of!(rec, length), vec![10, 10, 20, 10]);      
    /// ```            
    pub fn push(&mut self, mut field: Field) {
        // copy current length
        let length = self.flist.len();

        // set field index
        field.index = length;

        // offset at this moment is merely the length of record (starts at 0)
        field.offset_from_origin = self.calculated_length;

        // and adjust field bounds, depending on how the field was defined
        match field.creation_type {
            FieldCreationType::ByLength => {
                // calculate bounds
                field.lower_offset = field.offset_from_origin;
                field.upper_offset = field.offset_from_origin + field.length - 1;

                // and new record length
                self.calculated_length += field.length;
            }
            FieldCreationType::ByOffset => {
                // now length is the greastest bound value
                self.calculated_length = field.upper_offset + 1;
            }
        };

        // get last field having the same name (if any)
        match self.get(&field.name) {
            Some(ref mut v) => {
                field.multiplicity = v.pop().unwrap().multiplicity + 1;
            }
            None => (),
        }

        // build field ID
        field.id = format!("{}{}", field.name, field.multiplicity);

        // finally, save Field struct
        self.flist.push(field);
    }

    /// Tests whether a Record contains a Field by giving its name.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///
    /// let mut rec = set_up_by_length::<AsciiMode>();
    ///
    /// assert!(rec.contains_field("FIELD1"));
    /// assert!(!rec.contains_field("FOO"));    
    ///    
    /// ```     
    pub fn contains_field(&self, fname: &str) -> bool {
        self.flist.iter().any(|f| f.name == fname)
    }

    /// Returns the number of fields in the record.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///
    /// let mut rec = set_up_by_length::<AsciiMode>();
    /// assert_eq!(rec.count(), 4);
    ///    
    /// ```     
    pub fn count(&self) -> usize {
        self.flist.len()
    }

    /// Returns a vector of fields matching the predicate.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();    
    ///
    /// let fields = rec.filter(|f| f.length >= 20);
    /// assert_eq!(fields.unwrap().len(), 1);
    ///
    /// let fields = rec.filter(|f| f.length >= 30);
    /// assert!(fields.is_none());    
    /// ```      
    pub fn filter<F>(&self, pred: F) -> Option<Vec<&Field>>
    where
        F: Fn(&Field) -> bool,
    {
        // search for indices matching predicate. Return is a vector of field refs
        let result: Vec<_> = self.flist.iter().filter(|e| pred(e)).collect();

        match result.is_empty() {
            true => None,
            false => Some(result),
        }
    }

    /// Returns a vector of fields matching the field name (this returns a vector
    /// because a Field could appear more than once in a Record).
    ///
    /// # Examples
    /// ```
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();    
    ///
    /// assert_eq!(rec.get("FIELD1").unwrap().len(), 1);
    /// assert_eq!(rec.get("FIELD2").unwrap().len(), 2);
    /// assert_eq!(rec.get("FIELD3").unwrap().len(), 1);
    /// assert!(rec.get("FIELD4").is_none());           
    /// ```      
    pub fn get(&self, fname: &str) -> Option<Vec<&Field>> {
        self.filter(|f| f.name == fname)
    }

    /// Only keeps fields matching the predicate.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();    
    ///
    /// rec.retain(|f| f.length == 10);
    /// assert_eq!(rec.count(), 3);
    /// ```       
    pub fn retain<F>(&mut self, pred: F)
    where
        F: Fn(&Field) -> bool,
    {
        self.flist.retain(|e| pred(e))
    }

    /// Removes fields matching the predicate.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();    
    ///
    /// rec.remove(|f| f.length == 10);
    /// assert_eq!(rec.count(), 1);
    /// ```      
    pub fn remove<F>(&mut self, pred: F)
    where
        F: Fn(&Field) -> bool,
    {
        self.flist.retain(|e| !pred(e))
    }

    /// Returns the record value (concatenation of all field values).
    ///
    /// # Examples
    /// ```
    /// use rbf::vector_of;
    /// use rbf::record::ReadMode;
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();  
    ///
    /// let s = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
    /// rec.set_value(&s);
    ///
    /// assert_eq!(rec.value(), s.to_string());
    ///
    /// ```     
    pub fn value(&self) -> String {
        let v: Vec<_> = self.flist.iter().map(|f| f.raw_value.clone()).collect();
        v.join("")
    }

    /// Returns the value from a field matching the field name. If the field is duplicated, returns the
    /// first value.
    ///
    /// #panics
    /// If `fname` is not found.
    ///
    /// # Examples
    /// ```
    /// use rbf::vector_of;
    /// use rbf::record::ReadMode;
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();  
    ///
    /// let s = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
    /// rec.set_value(&s);
    ///
    /// assert_eq!(rec.get_value("FIELD1"), "AAAAAAAAAA");
    /// assert_eq!(rec.get_value("FIELD2"), "BBBBBBBBBB");
    /// assert_eq!(rec.get_value("FIELD3"), "CCCCCCCCCCCCCCCCCCCC");
    ///
    /// ```  
    ///
    /// ```should_panic
    /// use rbf::vector_of;
    /// use rbf::record::ReadMode;
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();  
    ///
    /// let s = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
    /// rec.set_value(&s);
    ///
    /// assert_eq!(rec.get_value("FOO"), "AAAAAAAAAA");
    ///
    /// ```      
    pub fn get_value(&self, fname: &str) -> &str {
        // check for key existence
        let fields = match self.get(fname) {
            Some(f) => f,
            None => panic!("key {} not found in record {}", fname, self.name),
        };

        // safely returns value
        fields[0].value()
    }

    /// Returns the value from a field when there're duplicated fields matching the
    /// field name. Returns the i-th field value (starting from 0).
    ///
    /// #panics
    /// If `fname` is not found.   
    ///
    /// # Examples
    /// ```
    /// use rbf::vector_of;
    /// use rbf::record::ReadMode;
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();  
    ///
    /// let s = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
    /// rec.set_value(&s);
    ///
    /// assert_eq!(rec.get_value_with_index("FIELD2",0), "BBBBBBBBBB");
    /// assert_eq!(rec.get_value_with_index("FIELD2",1), "DDDDDDDDDD");    
    ///
    /// ```  
    ///
    /// ```should_panic
    /// use rbf::vector_of;
    /// use rbf::record::ReadMode;
    /// use rbf::record::{AsciiMode, setup::set_up_by_length};
    ///     
    /// let mut rec = set_up_by_length::<AsciiMode>();  
    ///
    /// let s = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
    /// rec.set_value(&s);
    ///
    /// assert_eq!(rec.get_value_with_index("FOO",0), "AAAAAAAAAA");
    /// assert_eq!(rec.get_value_with_index("FIELD2",3), "AAAAAAAAAA");    
    ///
    /// ```      
    pub fn get_value_with_index(&self, fname: &str, i: usize) -> &str {
        // check for key existence
        let fields = match self.get(fname) {
            Some(f) => f,
            None => panic!("key {} not found in record {}", fname, self.name),
        };

        // check also index
        let f = match fields.get(i) {
            Some(f) => f,
            None => panic!(
                "index {} is out of bounds for field {} in record {}",
                i, fname, self.name
            ),
        };

        f.value()
    }

    /// Adjusts the line value to the record length. Use Cow to avoid string duplication
    /// when the value is not padded with blanks.
    fn adjust_value<'a>(&self, value: &'a str) -> Cow<'a, str> {
        // if shorter, left-pad with blanks
        if value.len() < self.calculated_length {
            //format!("{:length$} ", value, length=self.calculated_length)
            Cow::Owned(format!(
                "{:length$} ",
                value,
                length = self.calculated_length
            ))
        } else {
            Cow::Borrowed(value)
        }
    }

    /// Checks if record value matches combined field filter
    pub fn is_filter_matched(&self, filter: &RecordFilter) -> bool {
        let mut result = true;

        // check each of the record filters
        for f in &filter.expr {
            // get field value if any
            let fields = match self.get(&f.fname) {
                Some(f) => f,
                None => continue,
            };

            // if the same field name is found in the record, matches any
            result &= fields.iter().any(|x| x.is_filter_matched(f));
        }
        result
    }
}

/// Lists all field name and values from a Record.
///
/// # Examples
/// ```
/// use rbf::vector_of;
/// use rbf::record::ReadMode;
/// use rbf::record::{AsciiMode, setup::set_up_by_length};
///     
/// let mut rec = set_up_by_length::<AsciiMode>();  
///
/// let s = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
/// rec.set_value(&s);
///
/// assert_eq!(format!("{}", rec), "(FIELD1='AAAAAAAAAA',FIELD2='BBBBBBBBBB',FIELD3='CCCCCCCCCCCCCCCCCCCC',FIELD2='DDDDDDDDDD')");
/// ```   
impl<T> fmt::Display for Record<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: Vec<_> = self.flist.iter().map(|f| format!("{}", f)).collect();
        write!(f, "({})", s.join(","))
    }
}

// implement debug trait
impl<T> fmt::Debug for Record<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!(
            "name: <{}>, description: <{}>\n",
            self.name, self.description
        );
        for field in &self.flist {
            s += format!("\tfield:<{}>\n", field).as_str();
        }
        write!(f, "{}", s)
    }
}

/// Returns a field reference in record by giving its index within the record.
///
/// # Examples
///
/// ```should_panic
/// use rbf::vector_of;
/// use rbf::record::ReadMode;
/// use rbf::record::{AsciiMode, setup::set_up_by_length};
///     
/// let mut rec = set_up_by_length::<AsciiMode>();  
///
/// let v = rec[4].value();
///
/// ```
impl<T> Index<usize> for Record<T> {
    type Output = Field;

    fn index(&self, i: usize) -> &Self::Output {
        if i >= self.flist.len() {
            panic!(
                "record {}: index {} out of bounds, max index = {}",
                self.name,
                i,
                self.flist.len() - 1
            )
        }
        self.flist.index(i)
    }
}

/// Returns a mutable field reference in record by giving its index within the record.
///
/// # Examples
///
/// ```should_panic
/// use rbf::vector_of;
/// use rbf::record::ReadMode;
/// use rbf::record::{AsciiMode, setup::set_up_by_length};
///     
/// let mut rec = set_up_by_length::<AsciiMode>();  
///
/// let v = rec[4].value();
///
/// ```
impl<T> IndexMut<usize> for Record<T> {
    fn index_mut(&mut self, i: usize) -> &mut Field {
        if i >= self.flist.len() {
            panic!(
                "record {}: index {} out of bounds, max index = {}",
                self.name,
                i,
                self.flist.len() - 1
            )
        }
        self.flist.index_mut(i)
    }
}

// iterators for looping through fields of a record

/// consuming iterator
///
/// # Examples
/// ```
/// use rbf::vector_of;
/// use rbf::record::ReadMode;
/// use rbf::record::{AsciiMode, setup::set_up_by_length};
///     
/// let mut rec = set_up_by_length::<AsciiMode>();  
///
/// for f in rec {
///    assert!(f.length == 10 || f.length == 20)
/// }
///
/// ```    
impl<T> IntoIterator for Record<T> {
    type Item = Field;
    type IntoIter = ::std::vec::IntoIter<Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.flist.into_iter()
    }
}

/// non-consuming iterator (access items by ref)
///
/// # Examples
/// ```
/// use rbf::vector_of;
/// use rbf::record::ReadMode;
/// use rbf::record::{AsciiMode, setup::set_up_by_length};
///     
/// let mut rec = set_up_by_length::<AsciiMode>();  
///
/// for f in &rec {
///    assert!(f.length == 10 || f.length == 20)
/// }
///
/// ```
impl<'a, T> IntoIterator for &'a Record<T> {
    type Item = &'a Field;
    type IntoIter = Iter<'a, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.flist.iter()
    }
}

/// non-consuming mutable iterator (access items by mut ref)
///
/// # Examples
/// ```
/// use rbf::vector_of;
/// use rbf::record::ReadMode;
/// use rbf::record::{AsciiMode, setup::set_up_by_length};
///     
/// let mut rec = set_up_by_length::<AsciiMode>();  
///
/// for f in &mut rec {
///     f.length = 100;
/// }
///
/// for f in rec {
///    assert!(f.length == 100);
/// }
///
/// ```
impl<'a, T> IntoIterator for &'a mut Record<T> {
    type Item = &'a mut Field;
    type IntoIter = IterMut<'a, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.flist.iter_mut()
    }
}

/// Convenient conversion from a tuple.
///
/// # Examples
/// ```
/// use rbf::record::{AsciiMode, Record};
///     
/// let rec = Record::<AsciiMode>::from(("RECORD1", "Description for record 1", 20));  
///
/// assert_eq!(rec.name, "RECORD1");
///
/// ```
impl<'a, T> From<(&'a str, &'a str, usize)> for Record<T> {
    fn from(vals: (&'a str, &'a str, usize)) -> Record<T> {
        Record::new(vals.0, vals.1, vals.2)
    }
}

// module to setup test data for record
pub mod setup {
    use std::rc::Rc;

    use crate::field::Field;
    use crate::record::Record;
    use crate::types::fieldtype::FieldType;

    // this fn sets up the relevant data for testing a record, when fields are contiguous
    pub fn set_up_by_length<T>() -> Record<T> {
        let ft1 = Rc::new(FieldType::new("I", "int"));

        let f1 = Field::from_length("FIELD1", "Description for field 1", &ft1, 10);
        let f2 = Field::from_length("FIELD2", "Description for field 2", &ft1, 10);
        let f3 = Field::from_length("FIELD3", "Description for field 3", &ft1, 20);
        let f4 = Field::from_length("FIELD2", "Description for field 2", &ft1, 10);

        let mut rec = Record::<T>::new("RECORD1", "Description for record 1", 20);

        rec.push(f1);
        rec.push(f2);
        rec.push(f3);
        rec.push(f4);

        rec
    }

    // this fn sets up the relevant data for testing a record, when fields are not-contiguous
    pub fn set_up_by_offset<T>() -> Record<T> {
        let ft1 = Rc::new(FieldType::new("I", "int"));

        let f1 = Field::from_offset("FIELD1", "Description for field 1", &ft1, 5, 9);
        let f2 = Field::from_offset("FIELD2", "Description for field 2", &ft1, 15, 19);
        let f3 = Field::from_offset("FIELD3", "Description for field 3", &ft1, 30, 39);
        let f4 = Field::from_offset("FIELD2", "Description for field 2", &ft1, 50, 60);

        let mut rec = Record::<T>::new("RECORD1", "Description for record 1", 0);

        rec.push(f1);
        rec.push(f2);
        rec.push(f3);
        rec.push(f4);

        rec
    }

    // this fn sets up the relevant data for testing a hug record
    #[allow(unused_variables)]
    pub fn set_up_by_length_huge<T>(size: usize) -> Record<T> {
        let ft1 = Rc::new(FieldType::new("I", "int"));
        let mut rec = Record::<T>::new("RECORD1", "Description for record 1", 0);

        for i in 0..size {
            let f = Field::from_length("FIELD1", "Description for field 1", &ft1, 10);
            rec.push(f);
        }

        rec
    }

}

#[cfg(test)]
mod tests {

    use crate::record::{AsciiMode, ReadMode, UTF8Mode};

    #[test]
    fn record_ascii_by_length() {
        // setup data
        let mut rec = crate::record::setup::set_up_by_length::<AsciiMode>();

        assert_eq!(rec.calculated_length, 50);
        assert_eq!(rec.count(), 4);

        assert_eq!(
            vector_of!(rec, name),
            vec!["FIELD1", "FIELD2", "FIELD3", "FIELD2"]
        );
        assert_eq!(
            vector_of!(rec, description),
            vec![
                "Description for field 1",
                "Description for field 2",
                "Description for field 3",
                "Description for field 2"
            ]
        );
        assert_eq!(vector_of!(rec, length), vec![10, 10, 20, 10]);

        let s = "FIELD1".to_string();

        assert!(rec.contains_field(&s));
        assert_eq!(rec.contains_field("FOO"), false);

        assert!(rec.get("FIELD1").is_some());
        assert!(rec.get("FOO").is_none());

        // line has exactly the right length in chars
        let s2 = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDD";
        rec.set_value(&s2);
        assert_eq!(rec[0].value(), "AAAAAAAAAA");
        assert_eq!(rec[1].value(), "BBBBBBBBBB");
        assert_eq!(rec[2].value(), "CCCCCCCCCCCCCCCCCCCC");
        assert_eq!(rec[3].value(), "DDDDDDDDDD");
        assert_eq!(
            vector_of!(rec, raw_value),
            vec![
                "AAAAAAAAAA",
                "BBBBBBBBBB",
                "CCCCCCCCCCCCCCCCCCCC",
                "DDDDDDDDDD"
            ]
        );

        // test display
        assert_eq!(format!("{}", rec), "(FIELD1='AAAAAAAAAA',FIELD2='BBBBBBBBBB',FIELD3='CCCCCCCCCCCCCCCCCCCC',FIELD2='DDDDDDDDDD')");

        // line is over right length in chars
        let s3 = "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDEEEEEEEEEEEEEEEE";
        rec.set_value(&s3);
        assert_eq!(rec[0].value(), "AAAAAAAAAA");
        assert_eq!(rec[1].value(), "BBBBBBBBBB");
        assert_eq!(rec[2].value(), "CCCCCCCCCCCCCCCCCCCC");
        assert_eq!(rec[3].value(), "DDDDDDDDDD");
        assert_eq!(
            vector_of!(rec, raw_value),
            vec![
                "AAAAAAAAAA",
                "BBBBBBBBBB",
                "CCCCCCCCCCCCCCCCCCCC",
                "DDDDDDDDDD"
            ]
        );

        // line is shorter than the length in chars
        let s4 = "ZZZZZZZZZZ";
        rec.set_value(&s4);
        assert_eq!(rec[0].value(), "ZZZZZZZZZZ");
        assert_eq!(rec[1].raw_value, "          ");
        assert_eq!(rec[2].raw_value, "                    ");
        assert_eq!(rec[3].raw_value, "          ");
        assert_eq!(rec[1].value(), "");
        assert_eq!(rec[2].value(), "");
        assert_eq!(rec[3].value(), "");
        assert_eq!(vector_of!(rec, str_value), vec!["ZZZZZZZZZZ", "", "", ""]);
    }

    #[test]
    fn record_ascii_by_offset() {
        // setup data
        let mut rec = crate::record::setup::set_up_by_offset::<AsciiMode>();

        assert_eq!(rec.calculated_length, 60);
        assert_eq!(rec.count(), 4);

        assert_eq!(
            vector_of!(rec, name),
            vec!["FIELD1", "FIELD2", "FIELD3", "FIELD2"]
        );
        assert_eq!(
            vector_of!(rec, description),
            vec![
                "Description for field 1",
                "Description for field 2",
                "Description for field 3",
                "Description for field 2"
            ]
        );
        assert_eq!(vector_of!(rec, length), vec![5, 5, 10, 11]);

        let s = "FIELD1".to_string();

        assert!(rec.contains_field(&s));
        assert_eq!(rec.contains_field("FOO"), false);

        assert!(rec.get("FIELD1").is_some());
        assert!(rec.get("FOO").is_none());

        // line has exactly the right length in chars
        let s1 = "    AAAAA     BBBBB          CCCCCCCCCC          DDDDDDDDDDD";
        rec.set_value(&s1);
        assert_eq!(rec[0].value(), "AAAAA");
        assert_eq!(rec[1].value(), "BBBBB");
        assert_eq!(rec[2].value(), "CCCCCCCCCC");
        assert_eq!(rec[3].value(), "DDDDDDDDDDD");
        assert_eq!(
            vector_of!(rec, raw_value),
            vec!["AAAAA", "BBBBB", "CCCCCCCCCC", "DDDDDDDDDDD"]
        );
    }

    #[test]
    fn record_utf8_by_offset() {
        // setup data
        let mut rec = crate::record::setup::set_up_by_offset::<UTF8Mode>();

        // line has exactly the right length in chars
        let s1 = "    ααααα     βββββ          γγγγγγγγγγ          δδδδδδδδδδδ";
        rec.set_value(&s1);
        assert_eq!(rec[0].value(), "ααααα");
        assert_eq!(rec[1].value(), "βββββ");
        assert_eq!(rec[2].value(), "γγγγγγγγγγ");
        assert_eq!(rec[3].value(), "δδδδδδδδδδδ");
        assert_eq!(
            vector_of!(rec, raw_value),
            vec![
                "ααααα",
                "βββββ",
                "γγγγγγγγγγ",
                "δδδδδδδδδδδ"
            ]
        );
    }

    #[test]
    fn record_utf8_by_length() {
        // setup data
        let mut rec = crate::record::setup::set_up_by_length::<UTF8Mode>();

        let s5 = "ααααααααααββββββββββγγγγγγγγγγγγγγγγγγγγδδδδδδδδδδ";
        rec.set_value(&s5);
        assert_eq!(rec[0].value(), "αααααααααα");
        assert_eq!(rec[1].value(), "ββββββββββ");
        assert_eq!(rec[2].value(), "γγγγγγγγγγγγγγγγγγγγ");
        assert_eq!(rec[3].value(), "δδδδδδδδδδ");
        assert_eq!(
            vector_of!(rec, raw_value),
            vec![
                "αααααααααα",
                "ββββββββββ",
                "γγγγγγγγγγγγγγγγγγγγ",
                "δδδδδδδδδδ"
            ]
        );
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn record_panic() {
        // setup data
        let rec = crate::record::setup::set_up_by_length::<AsciiMode>();

        // this should panic
        let v1 = rec.get_value("FOO");
        let v2 = rec.get_value_with_index("FIELD2", 2);
    }

}
