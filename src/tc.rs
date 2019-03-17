// Static function trait function example

use std::borrow::ToOwned;
use std::string::ToString;

pub trait Formatter<T> {
    fn fmt(a: &T) -> String;
}

impl Formatter<Self> for &str {
    fn fmt(a: &Self) -> String {
        "[string: ".to_owned() + &a + "]"
    }
}

impl Formatter<Self> for i32 {
    fn fmt(a: &Self) -> String {
        "[int_32: ".to_owned() + &a.to_string() + "]"
    }
}

impl<T: Formatter<T>> Formatter<Self> for Vec<T> {
    fn fmt(a: &Self) -> String {
        a.iter().map(|e| T::fmt(e)).collect::<Vec<_>>().join(" :: ")
    }
}

pub fn fmt2<T>(t: T) -> String where T: Formatter<T> {
    T::fmt(&t)
}
