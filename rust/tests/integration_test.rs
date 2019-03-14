// main.rs
use rbf::reader::{Reader, ReaderLazyness};
use rbf::record::{AsciiMode, UTF8Mode};
use rbf::vector_of;

// module to setup test data for layout
pub mod setup {

    use rbf::layout::Layout;
    use rbf::record::{AsciiMode, UTF8Mode};

    pub fn layout_load_layout_ascii() -> Layout<AsciiMode> {
        // load our layout
        Layout::<AsciiMode>::new("./tests/test.xml").unwrap()
    }

    pub fn layout_load_layout_utf8() -> Layout<UTF8Mode> {
        // load our layout
        Layout::<UTF8Mode>::new("./tests/test.xml").unwrap()
    }

}

#[test]
fn record_filter() {
    // loop through r_ll record
    let layout = setup::layout_load_layout_ascii();
    let r_ll = layout.get("LL").unwrap();

    assert_eq!(r_ll.calculated_length, 353);

    // all record 01 field types has only AN, I, D or N
    let types = ["A".to_string(), "N".to_string()];
    for f in r_ll {
        // field length is less thant 400
        assert!(f.len() <= 26);

        // only those types
        assert!(types.contains(&f.ftype.id));

        // all field name have only 4 chars
        assert!(f.name.len() <= 4);
    }

    // filter out fields: only 2 fields greater than 30 chars
    let fields = r_ll.filter(|f| f.length >= 25);
    assert_eq!(fields.unwrap().len(), 2);

    // find duplicated: 5 occurences
    let r_dup = layout.get("DP").unwrap();
    let f_dup = r_dup.filter(|f| f.name == "F5").unwrap();
    assert_eq!(f_dup.len(), 4);

    // check multiplicity
    for (i, f) in f_dup.iter().enumerate() {
        assert_eq!(f.multiplicity, i);
    }
}

#[test]
fn record_remove() {
    // loop through r_ll record
    let mut layout = setup::layout_load_layout_ascii();

    // remove one field only
    {
        let r_ll = layout.get_mut("LL").unwrap();

        // remove first field
        r_ll.remove(|f| f.index == 0);
        assert_eq!(r_ll[0].name, "W1");
        assert_eq!(r_ll.count(), 26);

        // remove all F1* fields
        r_ll.remove(|f| f.name.starts_with("W1"));
        assert_eq!(r_ll.count(), 15);

        // remove all fields but SQNR
        r_ll.remove(|f| f.name != "W2");
        assert_eq!(r_ll.count(), 1);
    }

    {
        let r_nb = layout.get_mut("NB").unwrap();
        r_nb.remove(|f| !["N1", "N2"].contains(&&*f.name));
        assert_eq!(r_nb.count(), 2);
    }
}

#[test]
fn record_iterator() {
    // loop through r_ll record
    let mut layout = setup::layout_load_layout_ascii();

    // remove one field only

    // non-consuming iterator (r_ll is already a ref)
    {
        let r_ll = layout.get("LL").unwrap();

        // check out loops
        for f in r_ll {
            assert!(f.length < 27);
        }
    }

    // non-consuming mutable iterator (r_ll is already a ref)
    {
        {
            let r_ll = layout.get_mut("LL").unwrap();

            // check out loops
            for f in r_ll {
                f.length = 10;
            }
        }

        let r_ll = layout.get("LL").unwrap();
        let count = r_ll.count();
        let sum: usize = vector_of!(r_ll, length).iter().sum();
        assert_eq!(sum, 10 * count);
    }
}

#[test]
fn field_multiplicity() {
    // load our layout
    let layout = setup::layout_load_layout_ascii();

    // find r_dp duplicated: 10 occurences
    let r_dp = layout
        .get("DP")
        .unwrap()
        .filter(|f| f.name == "F5")
        .unwrap();
    assert_eq!(r_dp.len(), 4);

    // check multiplicity
    for (i, f) in r_dp.iter().enumerate() {
        assert_eq!(f.multiplicity, i);
    }
}

#[should_panic]
#[allow(unused_variables)]
#[test]
fn reader_stringent() {
    // load our layout
    let layout = setup::layout_load_layout_ascii();

    // create reader
    let mut reader = Reader::<AsciiMode>::new("./tests/test_ascii.data", layout);

    // set stringent mode
    reader.set_lazyness(ReaderLazyness::Strict);

    // at some point, this should panic
    while let Some(rec) = reader.next() {}
}

#[test]
fn reader_lazy() {
    // load our layout
    let layout = setup::layout_load_layout_utf8();

    // create reader
    let mut reader = Reader::<UTF8Mode>::new("./tests/test_utf8.data", layout);

    // useful vars
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "123456789";
    let greek = "αβγδεζηθικλμνξοπρστυφχψω";

    // read file and loop through records
    while let Some((_stats, rec)) = reader.next() {
        match rec.name.as_ref() {
            "LL" => {
                assert_eq!(rec.get_value("ID"), "LL");

                // test every field
                for (i, l) in letters.chars().enumerate() {
                    let fname = format!("W{}", i + 1);
                    assert_eq!(rec.get_value(&fname), l.to_string().repeat(i + 1));
                }
            }
            "NB" => {
                assert_eq!(rec.get_value("ID"), "NB");
                // test every field
                for (i, n) in digits.chars().enumerate() {
                    let fname = format!("N{}", i + 1);
                    assert_eq!(rec.get_value(&fname), n.to_string().repeat(i + 1));
                }
            }
            "GL" => {
                assert_eq!(rec.get_value("ID"), "GL");
                for (i, l) in greek.chars().enumerate() {
                    let fname = format!("G{}", i + 1);
                    assert_eq!(rec.get_value(&fname), l.to_string().repeat(i + 1));
                }
            }
            "DP" => {
                assert_eq!(rec.get_value("ID"), "DP");
                assert_eq!(rec.get("F5").unwrap()[0].value(), "AAAAA");
                assert_eq!(rec.get("F5").unwrap()[1].value(), "BBBBB");
                assert_eq!(rec.get("F5").unwrap()[2].value(), "CCCCC");
                assert_eq!(rec.get("F5").unwrap()[3].value(), "DDDDD");
            }
            _ => panic!(
                "record name <{}> not found in file <{}>",
                rec.name, "./tests/test.data"
            ),
        }
    }
}
