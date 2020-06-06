#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct Deep(Structure); //struct within a struct

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    //Debug
    // All std library types automatically are printable with {:?} too
    // but they don't look as good as they could be
    //      println!("Let's see just the first layer printed: {:?}", Structure(3));
    //      println!("Now, let's see struct-ception: {:?}", Deep(Structure(7)));

    //pretty printing - {:#?}
    let name = "Noah";
    let age = 22;
    let noah = Person {name, age};
    println!("{:#?}", noah)
}