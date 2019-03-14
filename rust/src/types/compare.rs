use std::marker::PhantomData;
use std::str::FromStr;

pub struct Compare<T> {
    system_type: PhantomData<T>,
}

type CompareFunc<T> = fn(&T, &T) -> bool;
type ConvertFunc<T> = fn(&str) -> T;

impl<T> Compare<T>
where
    T: PartialOrd + FromStr + Sized,
{
    /// Converts a string to type T.
    fn to_t(value: &str) -> T {
        let converted = match value.parse::<T>() {
            Ok(v) => v,
            Err(_) => panic!("unable to convert string value {}", value),
        };
        converted
    }

    /// Compares two strings according to the comparison function.
    /// Convert strings first to the associated type Output.
    fn compare(lhs: &str, rhs: &str, cnv: ConvertFunc<T>, cmp: CompareFunc<T>) -> bool {
        let l = cnv(lhs);
        let r = cnv(rhs);
        cmp(&l, &r)
    }

    /// Converts strings before comparing for equality.
    pub fn eq(lhs: &str, rhs: &str) -> bool {
        Self::compare(lhs, rhs, Self::to_t, T::eq)
    }

    /// Converts strings before comparing for less than.
    pub fn lt(lhs: &str, rhs: &str) -> bool {
        Self::compare(lhs, rhs, Self::to_t, T::lt)
    }

    /// Converts strings before comparing for greater than.
    pub fn gt(lhs: &str, rhs: &str) -> bool {
        Self::compare(lhs, rhs, Self::to_t, T::gt)
    }
}
