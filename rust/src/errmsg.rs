/// Macro
#[macro_export]
macro_rules! error_msg {
    ($msg: ident) => {{
        use crate::errmsg::$msg;
        format!("{}:{}", stringify!($msg), $msg)
    }};
    ($msg: ident, $($arg:tt)+) => {{
        use crate::errmsg::$msg;
        format!("{}:{}", stringify!($msg), $msg, $($arg)+)
    }};
}

// test only
pub const MSG0000: &'static str = "test message";

pub const MSG0100: &'static str = "cannot create a FieldDataType with empty id!";
pub const MSG0101: &'static str = "cannot create FieldDataType with an empty string type!";

// field
pub const MSG0110: &'static str = "cannot create a field with an empty name!";
pub const MSG0111: &'static str = "cannot create Field with a null length!";
pub const MSG0112: &'static str = "error creating field {}: lower offset {} > upper offset {}!";

//
//pub const MSG0111: &'static str = "error creating field {}: lower offset {} > upper offset {}!";
