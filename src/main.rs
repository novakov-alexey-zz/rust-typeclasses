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

fn fmt<T>(t: T) -> String where T: Formatter<T> {
    t.fmt()
}

fn main() {
    let x = fmt("Hello, world!");
    let i = fmt(4);
    let ii = 4.fmt();
    //let f: String = fmt(4.0);

    println!("{}", x);
    println!("{}", i);
    println!("{}", ii);
}
