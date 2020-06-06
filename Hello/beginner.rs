//compile ex: rustc beginner.rs
//execute binary ./beginner
fn main() {
    println!("Hello World :^)"); // ! calls a macro. Drop the ! and it'd call a function
    let x = 420;
    println!("Is 'x' 69 or 420? x = {}", x);

    // Formatted printing
    // format! - write formatted text to String.
    // print! - same as format! but the text is printed to the console.
    // println! - same as print! but a newline is appended.
    // eprint! - same as format! but the text is printed to the standard error.
    // eprintln! - same as eprint! but a newline is appended.
    // Rust checks formatting correctness at compile time!

    println!("{} slatts", 1629);
    println!("{0} {1}, {2} {3}", "On", "God", "No", "Cap"); //positional string arguments
    println!("{first} {second}",first="Yessirrr", second="!"); //named string arguments
    println!("{number:>width$}", number=1, width=6);
    #[derive(Debug)] //derive attribute automatically creates the implementation required to make this struct printable. W/o it, it's unprintable.
    struct Structure(i32); //32 signed int
    println!("This struct '{:?}' will print...", Structure(3)); //printing 3 passed into structure
}