//! Represents a structure containing all fields and records, read from an external XML file.
//! This definition file, called **layout** file, defines all field and record features.
//!
//! # Examples
//! ```rust
//!  use rbf::record::AsciiMode;
//!  use rbf::layout::Layout;
//!
//!  // load my layout
//!  let layout = Layout::<AsciiMode>::new("./tests/test.xml").unwrap();
//!
//!  // check layout methods
//!  assert_eq!(layout.contains_record("LL"), true);
//!  assert_eq!(layout.contains_record("100"), false);
//!
//!  // layout has 36 records
//!  assert_eq!(layout.len(), 4);
//!
//!  // first record has 27 fields
//!  assert_eq!(layout.get("LL").unwrap().count(), 27);
//!
//!  // fiel W1 is present in layout, but no FOO
//!  assert!(layout.contains_field("W1"));
//!  assert!(!layout.contains_field("FOO"));
//!
//!  // loop
//!  for (recname, rec) in &layout {
//!      assert!(recname.len() >= 2);
//!  }
//! ```
use std::fs::File;
//use std::error::Error;
use std::collections::HashMap;
use std::io::BufReader;
use std::rc::Rc;

use xml::reader::{EventReader, XmlEvent};

use crate::field::Field;
use crate::mapper::RecordMapper;
use crate::record::Record;
use crate::types::fieldtype::FieldType;
//use util::into_field_list;
use crate::error::{RbfError, Result};

// useful macro to get value from attribute name
#[doc(hidden)]
#[macro_export]
macro_rules! get_value {
    ($attr:ident, $name: expr) => {{
        $attr
            .iter()
            .find(|e| e.name.local_name == $name)
            .unwrap()
            .value
            .clone()
    }};
}

//#[derive(Debug)]
pub struct Layout<T> {
    /// XML layout file name
    pub xml_file: String,
    /// If all records have the same length, this stores the record length for all records
    pub rec_length: usize,
    /// Layout file version
    pub version: String,
    /// Layout file description   
    pub description: String,
    /// SQL schema name (future use)
    pub schema: String,
    /// Hash map of all read records from file
    pub rec_map: HashMap<String, Record<T>>,
    /// Hash map of all field types found when reading
    pub ftypes: HashMap<String, Rc<FieldType>>,
    // closure which maps each line to a record ID
    pub mapper: RecordMapper,
}

use xml::attribute::OwnedAttribute;
fn as_hash(attributes: &Vec<OwnedAttribute>) -> HashMap<&str, &str> {
    // loop through attributes to create a hash. Not present in xml_rs (?!)
    let mut h: HashMap<&str, &str> = HashMap::new();

    for own_attr in attributes {
        h.insert(&own_attr.name.local_name, &own_attr.value);
    }
    h
}

impl<T> Layout<T> {
    /// Reads the XML layout file to create record and field structs.
    ///
    /// # Arguments
    ///
    /// * `xml_file` - full file name and path of the XML layout file
    ///
    ///
    /// # Panics
    /// If `xml_file` could not be read   
    pub fn new(xml_file: &str) -> Result<Layout<T>> {
        // try to open xml_file
        let file = match File::open(&xml_file) {
            Ok(file) => BufReader::new(file),
            Err(e) => return Err(RbfError::ErrorOpeningLayoutFile(xml_file.to_string(), e)),
        };

        // define hash to hold fieldtypes
        let mut ftypes: HashMap<String, Rc<FieldType>> = HashMap::new();

        //let mut rec_list: Vec<Record> = Vec::new();
        let mut rec_map: HashMap<String, Record<T>> = HashMap::new();

        let mut last_rec_name = String::new();

        // temp variables to get meta values
        let mut rec_length: usize = 0;
        let mut version = String::new();
        let mut description = String::new();
        let mut schema = String::new();

        // temp variables for map
        let mut mtype = String::new();
        let mut domain = String::new();

        // loop through elements
        let parser = EventReader::new(file);
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    // fetch attributes as a hash
                    let attr = as_hash(&attributes);

                    // now depending on XML tag
                    match name.local_name.as_ref() {
                        "meta" => {
                            rec_length = match attr.get("reclength") {
                                Some(v) => v.parse::<usize>().unwrap(),
                                None => 0,
                            };

                            version = Layout::<T>::extract_attr(&attr, "version");
                            description = Layout::<T>::extract_attr(&attr, "description");
                            schema = Layout::<T>::extract_attr(&attr, "schema");
                        }
                        "map" => {
                            mtype = Layout::<T>::extract_attr(&attr, "type");
                            domain = Layout::<T>::extract_attr(&attr, "domain");
                        }
                        "fieldtype" => {
                            // mandatory XML attributes
                            let ft_name = attr.get("name").unwrap();
                            let ft_type = attr.get("type").unwrap();
                            let mut ft = FieldType::new(ft_name, ft_type);

                            // in some cases, a format is set
                            if let Some(v) = attr.get("format") {
                                ft.base_type.set_format(v);
                            }

                            // optional XML attributes
                            if let Some(v) = attr.get("pattern") {
                                ft.set_pattern(&v);
                            }

                            // finally insert field type
                            ftypes.insert(ft_name.to_string(), Rc::new(ft));
                        }
                        "record" => {
                            let rec_name = attr.get("name").unwrap();
                            let rec_desc = attr.get("description").unwrap();
                            let rec_length: usize;

                            // save last met Record name to be able to add fields whenever we meet
                            // a <field> tag
                            last_rec_name = rec_name.to_string();

                            // length could be present or Not
                            rec_length = match attr.get("length") {
                                Some(length) => length.parse::<usize>().unwrap(),
                                None => 0,
                            };

                            // add new record
                            //rec_list.push(Record::new(last_rec_name, rec_type, rec_length))
                            rec_map.insert(
                                rec_name.to_string(),
                                Record::<T>::new(rec_name, rec_desc, rec_length),
                            );
                        }
                        "field" => {
                            // name and description are mandatory
                            let f_name = attr.get("name").unwrap();
                            let f_desc = attr.get("description").unwrap();

                            // so is the field type
                            let f_type = attr.get("type").unwrap().to_string();

                            // try to get already insert field type
                            let ft = match ftypes.get(&f_type) {
                                Some(ft) => ft,
                                None => {
                                    return Err(RbfError::ErrorLayoutNoFieldType(
                                        xml_file.to_string(),
                                        f_name.to_string(),
                                        f_type,
                                    ));
                                }
                            };

                            // length could be present or Not
                            let f_length = match attr.get("length") {
                                Some(length) => length.parse::<usize>().unwrap(),
                                None => 0,
                            };

                            // if length is not present, then lower and upper bounds for this field should
                            // be present
                            if f_length == 0 {
                                // get lower offset
                                let f_lower_offset = match attr.get("start") {
                                    Some(n) => n.parse::<usize>().unwrap(),
                                    None => 0,
                                };

                                // get upper offset
                                let f_upper_offset = match attr.get("end") {
                                    Some(n) => n.parse::<usize>().unwrap(),
                                    None => 0,
                                };

                                // add Field into the last created record
                                rec_map
                                    .get_mut(&last_rec_name)
                                    .unwrap()
                                    .push(Field::from_offset(
                                        f_name,
                                        f_desc,
                                        &ft,
                                        f_lower_offset,
                                        f_upper_offset,
                                    ));
                            }
                            // here, length is not null
                            else {
                                // add Field into the last created record
                                rec_map
                                    .get_mut(&last_rec_name)
                                    .unwrap()
                                    .push(Field::from_length(f_name, f_desc, &ft, f_length));
                            }
                        }
                        _ => (),
                    }
                    //println!("{} {:?}", name, attributes);
                }
                Err(e) => {
                    return Err(RbfError::ErrorReadingLayoutFile(xml_file.to_string(), e));
                    //break;
                }
                _ => {}
            }
        }

        Ok(Layout {
            xml_file: xml_file.to_string(),
            rec_length: rec_length,
            version: version,
            description: description,
            schema: schema,
            rec_map: rec_map,
            ftypes: ftypes,
            mapper: RecordMapper::new(&mtype, &domain),
        })
    }

    /// Returns the number of records in the layout.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///     
    /// let layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// assert_eq!(layout.len(),4);
    ///
    /// ```      
    pub fn len(&self) -> usize {
        self.rec_map.len()
    }

    /// Tests if Layout contains a record by giving its name.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///     
    /// let layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// assert_eq!(layout.contains_record("LL"), true);
    /// assert_eq!(layout.contains_record("100"), false);
    ///
    /// ```     
    pub fn contains_record(&self, recname: &str) -> bool {
        self.rec_map.contains_key(recname)
    }

    /// Tests if Layout contains a field record-wise.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///     
    /// let layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// assert!(layout.contains_field("W1"));
    /// assert!(!layout.contains_field("FOO"));
    ///
    /// ```     
    pub fn contains_field(&self, fname: &str) -> bool {
        self.rec_map.iter().any(|(_, v)| v.contains_field(fname))
    }

    /// Gets a record reference from its name.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///     
    /// let layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// assert_eq!(layout.get("LL").unwrap().count(), 27);
    ///
    /// ```     
    pub fn get(&self, rec_name: &str) -> Option<&Record<T>> {
        self.rec_map.get(rec_name)
    }

    /// Gets a mutable reference on record from its name.   
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///     
    /// let mut layout = layout_load_layout_ascii("./tests/test.xml");
    /// let mut rec = layout.get_mut("LL").unwrap();
    ///
    /// rec.description = String::from("A new field description");
    ///
    /// ```    
    pub fn get_mut(&mut self, rec_name: &str) -> Option<&mut Record<T>> {
        self.rec_map.get_mut(rec_name)
    }

    /// Gets a field type Rc.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///     
    /// let layout = layout_load_layout_ascii("./tests/test.xml");
    /// let ft = layout.get_type("A").unwrap();
    ///
    /// assert_eq!(ft.pattern.as_str(), "\\w+");
    ///
    /// ```       
    pub fn get_type(&self, ftype_name: &str) -> Option<&Rc<FieldType>> {
        self.ftypes.get(ftype_name)
    }

    /// Removes each field from the list from the whole layout, i.e. from all records.
    /// If a field name doesn't exist, no error is returned and the deletion is ignored.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///     
    /// let mut layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// assert_eq!(layout.contains_field("ID"), true);
    ///
    /// // remove all "ID" fields from all records
    /// layout.remove(vec!["ID"]);
    /// assert_eq!(layout.contains_field("ID"), false);
    ///
    /// // remove a list
    /// layout.remove(vec!["W26", "N9", "G24"]);
    /// assert_eq!(layout.contains_field("ID"), false);
    ///
    /// assert_eq!(layout.get("LL").unwrap().count(), 25);
    /// assert_eq!(layout.get("NB").unwrap().count(), 8);
    /// assert_eq!(layout.get("GL").unwrap().count(), 23);
    ///
    /// ```      
    pub fn remove(&mut self, flist: Vec<&str>) {
        for (_, rec) in &mut self.rec_map {
            rec.remove(|f| flist.contains(&&*f.name));
        }
    }

    /// Retains only the records and list specified. All other records or fields are removed.
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///     
    /// // load our layout
    /// let mut layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// // build list of rec/fields to retain
    /// use std::collections::HashMap;
    /// let mut rec_map: HashMap<&str, Vec<&str>> = HashMap::new();
    /// rec_map.insert("LL", vec!["ID"]);
    /// rec_map.insert("NB", vec!["ID", "N2"]);
    ///
    /// // prune records and fields
    /// layout.retain(rec_map);
    ///
    /// assert_eq!(layout.get("LL").unwrap().count(), 1);
    /// assert_eq!(layout.get("NB").unwrap().count(), 2);
    /// assert_eq!(layout.contains_record("GL"), false);
    ///
    /// ```      
    pub fn retain(&mut self, rec_list: HashMap<&str, Vec<&str>>) {
        // create vector of record names to retain only those ones.
        let rec_names: Vec<_> = rec_list.keys().collect();
        self.rec_map.retain(|ref k, _| rec_names.contains(&&&***k));

        // now for each remaining record, delete given fields
        for (rec_name, rec) in &mut self.rec_map {
            if rec_names.contains(&&&**rec_name) {
                rec.retain(|f| rec_list.get(&**rec_name).unwrap().contains(&&*f.name));
            }
        }
    }

    /// Checks whether layout is valid: if `rec_length` is not 0, all records have the same length
    /// the sum of length all fields (i.e. record length) should match the `rec_length` value.
    /// If not, each record length should match the declared length
    ///
    /// # Examples
    /// ```
    /// use rbf::record::AsciiMode;
    /// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
    ///    
    /// let mut layout = layout_load_layout_ascii("./tests/test.xml");
    ///
    /// assert!(layout.is_valid().0);
    ///
    ///
    ///
    /// ```       
    pub fn is_valid(&self) -> (bool, &str, usize, usize) {
        if self.rec_length != 0 {
            for (_, rec) in &self.rec_map {
                if self.rec_length != rec.calculated_length {
                    return (false, "", self.rec_length, rec.calculated_length);
                }
            }
        } else {
            for (_, rec) in &self.rec_map {
                if rec.declared_length != rec.calculated_length {
                    return (false, &rec.name, rec.declared_length, rec.calculated_length);
                }
            }
        }
        (true, "", 0, 0)
    }

    /// Sets skip field.
    /*    pub fn set_skip_field(&mut self, skip_field: &str) {
        // save value and delete all fields in the list from layout
        self.skip_field = String::from(skip_field);

        // remove field names
        self.remove(into_field_list(skip_field));
    }*/

    /// Private func: extracts the value of a string XML attribute.
    /// If not found, just returns a new string.
    fn extract_attr(attr: &HashMap<&str, &str>, attr_name: &str) -> String {
        let extraction = match attr.get(attr_name) {
            Some(v) => v.to_string(),
            None => String::from(""),
        };
        extraction
    }
}

/// non-consuming iterator (access items by ref)
///
/// # Examples
///
/// ```
/// use rbf::record::AsciiMode;
/// use rbf::layout::{Layout, setup::layout_load_layout_ascii};    
///    
/// let layout = layout_load_layout_ascii("./tests/test.xml");
///
/// for (recname, rec) in &layout {
///     assert!(recname.len() >= 2);
///     assert!(rec.name.len() <= 3);
/// }
///
/// ```   
impl<'a, T> IntoIterator for &'a Layout<T> {
    type Item = (&'a String, &'a Record<T>);
    type IntoIter = ::std::collections::hash_map::Iter<'a, String, Record<T>>;

    // a Record contains a vector, just return the vector iterator
    fn into_iter(self) -> Self::IntoIter {
        self.rec_map.iter()
    }
}

// module to setup test data for layout
pub mod setup {

    use crate::layout::Layout;
    use crate::record::AsciiMode;

    pub fn layout_load_layout_ascii(test_file: &str) -> Layout<AsciiMode> {
        // load our layout
        Layout::<AsciiMode>::new(test_file).unwrap()
    }

}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn layout_skip_field() {
//         // load our layout
//         let mut layout = crate::layout::setup::layout_load_layout_ascii("./tests/test.xml");

//         let flist: Vec<_> = "ID , W26,    N9 ,   G24 "
//             .split(",")
//             .map(|f| f.trim())
//             .collect();
//         layout.remove(flist);

//         assert_eq!(layout.contains_field("ID"), false);
//         assert_eq!(layout.get("LL").unwrap().count(), 25);
//         assert_eq!(layout.get("NB").unwrap().count(), 8);
//         assert_eq!(layout.get("GL").unwrap().count(), 23);
//     }

// }
