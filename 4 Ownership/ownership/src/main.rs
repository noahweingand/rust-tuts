fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) //returning multiple values in a tuple
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(a_string: &mut String) {
    a_string.push_str("I'd rather die before I come in last...");
}

fn main() {
    // Ownership
    
    let mut s = String::from("Hello"); //String data type, not string literal
    // :: - an operator that allows us to namespace this particular 'from' function under String
    // rather than using some sort of name like 'string_from'.
    
    s.push_str(", world!"); //push_str() appends a literal to string (mutating previous String)
    println!("{}", s);

    //copying Strings
    let s1 = String::from("hello");
    let s2 = s1;
    // s2 copies the String data, meaning we copy the pointer, the length, and the capacity that
    // are on the stack. No data on the heap is copied. With both data pointers pointing to the
    // same location, they will both try to free the same memory. This known as a 'double error'
    // and is one of the memory safe bugs. Freeing memory twice can lead to memory corruption,
    // which can potentially leading to security vulnerabilities.
    // Something else to note here is that Rust considers s1 to no longer be valid and, therefore,
    // Rust doesn't need to free anything when s1 goes out of scope. It's known as a 'move'.
    // s1 was 'moved' to s2. 

    let s3 = String::from("whaddup!");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    let (s5, len) = calculate_length(s3);
    println!("The length of '{}' is {}.", s5, len);

    // References

    let s6 = String::from("reference example");
    let len = calculate_length2(&s6);
    println!("The length of '{}' is {}.", s6, len);

    //Mutable References

    let mut s7 = String::from("no cap and gown, i ain't go to class...");
    change(&mut s7);
    println!("{}",s7);
} 
// s is out of scope here
