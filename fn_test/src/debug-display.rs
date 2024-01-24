use std::fmt;

#[derive(Debug)]
struct MyStruct {
    field1: i32,
    field2: String,
}

#[derive(Debug)] // COMMENT THIS LINE WILL MAKE IT NotPrintable NOT PRINTABLE
struct NotPrintable(i32);

#[derive(Debug)]
struct Deep(NotPrintable);

struct MyStruct2(i32);

impl fmt::Display for MyStruct2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is a customized display of struct({},{})", self.0, self.0)
    }
}


fn main() {
    let my_struct = MyStruct {
        field1: 42,
        field2: String::from("Hello, world!"),
    };

    println!("{:?}", my_struct);
    println!("{:#?}", my_struct); // pretty print

    // println!("{:?}", NotPrintable(42);
    println!("{:?}", Deep(NotPrintable(42)));

    println!("{}", MyStruct2(42));

}