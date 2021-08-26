use std::env;

#[derive(Debug)]
struct Structure(u8);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let curDir = env::current_dir().unwrap();

    println!("Hello, world!\nCurrent Dir {0}", curDir.display());

    let tempStruct = Structure(42);

    let name = "Peter";
    let age = 16;
    let peter = Person { name, age};

    println!("{:?}", peter);
    println!("{:?}", tempStruct);
    println!("{:?}", Deep(Structure(tempStruct.0)));

    let mut thing : u8 = 5;
    thing += 1;
    println!("{value:0>4}", value=thing);
}
