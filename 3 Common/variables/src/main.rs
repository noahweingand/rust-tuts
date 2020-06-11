fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6;
    println!("The value of x is: {}", x);
    let mut y = 420;
    println!("The value of y is: {}", y);
    y = 666;
    println!("The value of y is now: {}", y);

    //constants
    //const MY_CONSTANT: u32 = 100_000; //underscores can be inserted in numeral literals for readability

    //shadowing example
    let x = 120;
    println!("The value of x is: {}", x);
    let x = x + 30;
    println!("The value of x is now: {}, thanks to shadowing", x);

    //tuples
    let tup: (i32, f64, u8) = (420, 6.6, 1);
    let another_tup = (500, 6.4, 1);
    let (x,y,z) = another_tup;
    println!("The values of another_tup are {}, {}, and {}", x,y,z);
    let last_tup = (147, "master chief");
    let (his_name, his_number) = last_tup;
    let his_name = last_tup.1;
    let his_number = last_tup.0;
    println!("{} is {}...", his_name, his_number);

    //arrays
    let an_array = [1,2,3,4,5];
    let another_array: [i32; 5] = [1,2,3,4,5];
    let array3 = [3; 5]; // 5 elements of the integer 3
    let index = array3[0];
    let element = array3[index];
    println!("Getting an element at index with variable: {}", element);

    //functions
    another_function();
    another_function2(2,4);
    println!("Example of a function returning: {}", return_this());
    println!("Another return ex: {}", slatt(5));

    //control flow
    let num = 1;
    if num < 2 {
        println!("this condition is true");
    }
    else {
        println!("this condition is false");
    }

    let condition = true;
    let a_number = if condition { 5 } else { 6 }; // if condition false, a_number will be 6.
    println!("The value of number is: {}", a_number);

    //loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result from loop: {}", result);

    let mut swag = 3;
    while swag != 0 {
        println!("{}!", swag);
        swag -= 1;
    }
    println!("Bomb detonated...");
    //or
    for a_num in (1..4).rev() {
        println!("{} !", a_num)
    }
    println!("Kaboom!!!");

    let nums = [10, 20, 30, 40 , 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", nums[index]);
        index += 1;
    }
    //or
    for element in nums.iter() {
        println!("The value is: {}", element)
    }
}

fn another_function() {
    println!("Hello from another function!!! :)");
}

fn another_function2(x: i32, y: i32) {
    println!("The values passed in x and y are: {} and {}", x, y);
}

fn return_this() -> i32 {
    1337
}

fn slatt(x: i32) -> i32 {
    x + 1
}