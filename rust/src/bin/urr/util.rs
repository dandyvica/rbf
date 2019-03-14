use std::collections::HashMap;

const FIELD_DELIMITER: char = ',';
const RECORD_FIELD_DELIMITER: char = ':';

/// Converts a comma-separated string into a vector of trimmed string refs.
/// # Example
/// ```rust
/// use rbf::util::into_field_list;
///
/// let mut s = into_field_list("AA, BB, CC, DD  ");
/// assert_eq!(s, vec!("AA","BB","CC","DD"));
/// ```
pub fn into_field_list(s: &str) -> Vec<&str> {
    let flist: Vec<_> = s.split(FIELD_DELIMITER).map(|f| f.trim()).collect();
    flist
}

/// Converts a pattern to a map of trimmed string refs. Key is the record name,
/// value is the vector of field names.
/// # Example
/// ```rust
/// use rbf::util::{into_field_list, into_rec_map};
///
/// let v = into_rec_map("F1:AA,  BB, CC ; F2: DD, EE, FF   ; F3: GG, HH  ", ";");
/// assert_eq!(v.get("F1").unwrap(), &vec!("AA","BB","CC"));
/// assert_eq!(v.get("F2").unwrap(), &vec!("DD","EE","FF"));
/// assert_eq!(v.get("F3").unwrap(), &vec!("GG","HH"));
/// ```
pub fn into_rec_map<'a>(s: &'a str, delimiter: &'static str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut rec_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for list in s.split(delimiter) {
        let v: Vec<_> = list
            .split(RECORD_FIELD_DELIMITER)
            .map(|f| f.trim())
            .collect();
        rec_map.insert(v[0], into_field_list(v[1]));
    }

    rec_map
}

// Converts a pattern into a list of field expression, meant to match field values.

/*pub fn into_field_regex(s: &str) -> Vec<FieldExpr> {
    // regex used to split expression
    let expr_reg = Regex::new(r"(?P<field>\w+)\s+(?P<op>[=~!<>]+)\s+(?P<re>.+)$").unwrap();

    // will hold result
    let mut vec: Vec<FieldExpr> = Vec::new();

    // split according to delimiter
    for expr in s.split(REGEX_DELIMITER) {
        let caps = expr_reg.captures(expr).unwrap();
        println!("caps={:?}", caps);
        let fexpr = FieldExpr{
            fname: caps["field"].to_string(),
            op: FieldExprOp::from(&caps["op"]),
            fregex: Regex::new(&caps["re"]).unwrap()
        };

        vec.push(fexpr);
    }

    vec
}*/
