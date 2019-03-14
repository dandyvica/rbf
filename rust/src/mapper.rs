use regex::Regex;

/// Convenient conversion from a string ref.
pub type RecordHasher = Box<Fn(&str) -> String>;

pub struct RecordMapper {
    pub func: RecordHasher,
}

/// Default closure is the identity function
impl Default for RecordMapper {
    fn default() -> RecordMapper {
        RecordMapper {
            func: Box::new(|x: &str| x.to_string()),
        }
    }
}

/// Builds the closure used to map a line to a record ID.
///
/// # Example
/// ```
/// use rbf::mapper::RecordMapper;
///
///  // our tst string
///  let s = "01XX02AAAAAAAAAAAAAAAAAAA";
///
///  // constant
///  let m1 = RecordMapper::new("constant", "FOO");
///  assert_eq!((m1.func)(s), "FOO");
///
///  // linear
///  let s = "01XX02AAAAAAAAAAAAAAAAAAA";
///  let m2 = RecordMapper::new("range", "0..2");
///  assert_eq!((m2.func)(s), "01");
/// ```
///
/// ```should_panic
/// use rbf::mapper::RecordMapper;
///
/// let m = RecordMapper::new("complex", "foo");
/// let s = "01XX02AAAAAAAAAAAAAAAAAAA";
/// ```
#[allow(unused_variables)]
impl RecordMapper {
    pub fn new(mtype: &str, domain: &str) -> RecordMapper {
        match mtype {
            "constant" => {
                // in this case, closure is just returning a constant string
                let dmn = domain.to_string();
                RecordMapper {
                    func: Box::new(move |x: &str| dmn.clone()),
                }
            }
            "range" => {
                // simple range
                let range_reg = Regex::new(r"(?P<r_inf>\d+)\.\.(?P<r_sup>\d+)").unwrap();
                let caps_range = range_reg.captures(domain).unwrap();
                let range = (
                    caps_range["r_inf"].parse::<usize>().unwrap(),
                    caps_range["r_sup"].parse::<usize>().unwrap(),
                );

                RecordMapper {
                    func: Box::new(move |x: &str| x[range.0..range.1].to_string()),
                }
            }
            "fancy" => unimplemented!(),
            _ => panic!("Unknown type pattern {}", mtype),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::mapper::RecordMapper;

//     #[test]
//     fn mapper_test() {
//         // our tst string
//         let s = "01XX02AAAAAAAAAAAAAAAAAAA";

//         // constant
//         let m1 = RecordMapper::new("constant", "FOO");
//         assert_eq!((m1.func)(s), "FOO");

//         // linear
//         let s = "01XX02AAAAAAAAAAAAAAAAAAA";
//         let m2 = RecordMapper::new("range", "0..2");
//         assert_eq!((m2.func)(s), "01");

//         // fancy
//         //let m3 = RecordMapper::new("type:2 map:0..2,4..6");
//         //assert_eq!((m3.func)(s), "0102");
//     }

//     #[test]
//     #[should_panic]
//     #[allow(unused_variables)]
//     fn field_badcons() {
//         let m = RecordMapper::new("complex", "foo");
//     }

// }
