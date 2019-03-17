use rust_typeclasses::tc::*;

pub trait Formatter<T> {
    fn fmt(&self) -> String;
}

impl Formatter<Self> for &str {
    fn fmt(&self) -> String {
        "[string: ".to_owned() + &self + "]"
    }
}

impl Formatter<Self> for i32 {
    fn fmt(&self) -> String {
        "[int_32: ".to_owned() + &self.to_string() + "]"
    }
}

impl<T: Formatter<T>> Formatter<Self> for Vec<T> {
    fn fmt(&self) -> String {
        self.iter().map(|e| e.fmt()).collect::<Vec<_>>().join(" :: ")
    }
}

fn fmt<T>(t: T) -> String where T: Formatter<T> {
    t.fmt()
}

fn main() {
    let x = fmt("Hello, world!");
    let i = 4.fmt();
    let ints = vec![1, 2, 3].fmt();
//    let floats = fmt(vec![1.0, 2.0, 3.0]);
//    let f: String = fmt(4.0);

    let s= fmt2("fmt 2");
    println!("{}", s);
    println!("{}", x);
    println!("{}", i);
    println!("{}", ints)
}
