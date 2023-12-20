// use rand::Rng;
// use std::cmp::Ordering;
use std::io;
// mod function;

fn main() {
    println!("Welcome to Rust!");
    // // 1. Square a number
    // let mut input_number = String::new();
    // println!("Enter a number to find Square of that number");
    // io::stdin()
    //     .read_line(&mut input_number)
    //     .expect("Please enter a number");
    // // let input: u32 = input_number.parse().expect("Wanted a number"); // without trim() the parse isn't working surprisingly
    // let input_number: u32 = input_number.trim().parse().expect("Wanted a number");
    // println!(
    //     "you entered {input_number} and square of it is {}",
    //     input_number * input_number
    // );

    // // 2.Guess the number
    // let secret_number = rand::thread_rng().gen_range(1..=10);
    // // println!("Generated number is {secret_number}");
    // loop {
    //     let mut input_number = String::new();
    //     println!("Please input your guess.");
    //     io::stdin()
    //         .read_line(&mut input_number)
    //         .expect("Failed to read line");
    //     println!("You guessed: {input_number}");
    //     // convert string into int32
    //     let input_number: u32 = input_number.trim().parse().expect("Wanted a number");

    //     match input_number.cmp(&secret_number) {
    //         Ordering::Less => println!("Go higher"),
    //         Ordering::Greater => println!("Gone too high"),
    //         Ordering::Equal => {
    //             println!("Yay! you won!");
    //             break;
    //         }
    //     }
    // }

    // // 3. Handling valid input
    // let secret_number = rand::thread_rng().gen_range(1..=2);
    // // println!("Generated number is {secret_number}");
    // loop {
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     println!("you've entered {guess}");
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Go higher"),
    //         Ordering::Greater => println!("Gone too high"),
    //         Ordering::Equal => {
    //             println!("Yay! you won!");
    //             break;
    //         }
    //     }
    // }

    // // 4. Variables & Mutability
    // let mut x = 5;
    // // this println to read the variable otherwise compiler will throw an error as
    // // warning: value assigned to `x` is never read
    // // its an edge case, arguably there's no necessacity to override the value of a variable before it gets read.
    // // go ahead and comment below println of Initial value of `x`
    // println!("Initial value of x is {x}");
    // x = 6;
    // println!("Overwritten value of x is {x}");

    // 5. Constants
    // const MY_AGE: u32 = 44;
    // println!("My age is {MY_AGE}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("THREE HOURS IN SECONDS is {THREE_HOURS_IN_SECONDS}");

    // // 6. Shadowing
    // let x = 1;
    // println!("value of x is {x}");
    // let x = x + 2; // if you see the `let` makes the variable overwritten without that we will get mutability compile-time error.
    // {
    //     let x = x + 2;
    //     println!("value of x is {x}");
    // }
    // println!("value of x is {x}");

    // let spaces = "    ";
    // println!("Value of the string is `{spaces}`");
    // let spaces = spaces.len(); // `let` allows to reuse the same variable name OR creates a new variable with same name.
    // println!("Length of the string is {spaces}");

    // 7. Data Types
    // let guess: u32 = "42".parse().expect("Not a number");
    // println!("value of guess is {guess}");

    // // 8. Scalar types
    // // 8.1 integers
    // let int_var_u8: u8 = "255".parse().expect("Not a u8");
    // println!("Value of int_var_u8 is {int_var_u8}");
    // let int_var_u16: u16 = "10250".parse().expect("Not a u16");
    // println!("Value of int_var_u16 is {int_var_u16}");
    // let int_var_u32: u32 = "255".parse().expect("Not a u32");
    // println!("Value of int_var_u32 is {int_var_u32}");
    // let int_var_u64: u64 = "255".parse().expect("Not a u64");
    // println!("Value of int_var_u64 is {int_var_u64}");
    // let int_var_u128: u128 = "255".parse().expect("Not a u128");
    // println!("Value of int_var_u128 is {int_var_u128}");

    // // 8.2 floating-point numbers
    // // Basically single precision floating point arithmetic deals with
    // // 32 bit floating point numbers whereas double precision deals with 64 bit.
    // // The number of bits in double precision increases the maximum value
    // // that can be stored as well as increasing the precision (ie the number of significant digits)

    // let x = 2.12345678901234567890;
    // println!("Value of x is {x}"); // default is 64 bit: can store upto 16 digits after decimal point
    // let y: f32 = 3.12345678901234567890;
    // println!("Value of y is {y}"); // 32 bit: can store upto 7 digits after decimal point

    // // 8.3 numeric operations
    // let sum = 5 + 5;
    // println!("Total is {sum}");

    // let diffr = 10 - 5;
    // println!("Difference is {diffr}");

    // let divide = 120 / 2;
    // println!("Divided value is {divide}");

    // let quotient = 12 / 7;
    // println!("quotient value is {quotient}");

    // let truncated = -5 / 3;
    // println!("truncated value is {truncated}");

    // let remainder = 12 % 7;
    // println!("remainder is {remainder}");

    // // 8.4 booleans
    // let t = true;
    // println!("value of t is {t}");

    // let f: bool = false;
    // println!("value of f is {f}");

    // // 8.5 characters
    // let c = 'z';
    // println!("Value of c is `{c}`");

    // let z: char = 'Z';
    // println!("Value of z is `{z}`");

    // let heart_eyed_cat = '😻';
    // println!("Value of heart_eyed_cat is `{heart_eyed_cat}`");

    // // 9. Compund types
    // let emp_tuple: (i32, f64, u8) = (23, 34.23, 8);
    // let (age, sal, dob) = emp_tuple;

    // println!("age    = {age}");
    // println!("salary = {sal}");
    // println!("dob    = {dob}");

    // // 10. Array type
    // // strongly typed array elements that allows only i32 numbers here
    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // let first = arr[0];
    // println!("First element value is {first}");

    // // we can keep same value in every element in an array using
    // let arr2: [i32; 5] = [5; 5];

    // let arr2_first = arr2[0];
    // println!("arr2 1st element value is {arr2_first}");

    // let arr2_second = arr2[1];
    // println!("arr2 2nd element value is {arr2_second}");

    // let arr2_third = arr2[2];
    // println!("arr2 3rd element value is {arr2_third}");

    // let arr2_fourth = arr2[3];
    // println!("arr2 4th element value is {arr2_fourth}");

    // let arr2_fifth = arr2[4];
    // println!("arr2 5th element value is {arr2_fifth}");

    // // let arr2_sixth = arr2[5]; // not exists
    // // println!("arr2 6th element value is {arr2_sixth}"); // should throw index out of bound error

    // // 11. Functions
    // // function::addition();
    // call_me("Satish", 'M', 44);
    // call_me("Madhu", 'F', 38)

    // // 12. Statements & Expressions
    // // let x = let y = 6; // error, coz statements wont return values.
    // let y = {
    //     let x = 3;
    //     x + 1 // expressions do not end with semicolons.
    // };
    // println!("value of y is {y}");

    // // 13. Functions with return values
    // let x = five();
    // println!("value of five is - {x}");
    // let y = plus_one(100);
    // println!("value of Y is - {y}");

    // // 14. control flow
    // // if statement
    // let loop_result = loop {
    //     let mut input_number = String::new();
    //     io::stdin()
    //         .read_line(&mut input_number)
    //         .expect("enter only a number.");
    //     let number: u32 = input_number.trim().parse().expect("expected only a number");

    //     let mut condition = false;
    //     if number > 5 {
    //         println!("number is greater than Five");
    //     } else if number < 5 {
    //         println!("number is lesser than Five");
    //     } else {
    //         println!("number is equal to Five");
    //         condition = true;
    //     }

    //     let condition_status = if condition {
    //         "Number is correct!"
    //     } else {
    //         "Wrong Nmber!"
    //     };
    //     // if in let statement
    //     if condition {
    //         // // 1. simple break without returning any value
    //         // println!("Condition value is: {condition_status}");
    //         // break;
    //         // // 2. break with return value from loop()
    //         break { condition_status };
    //     } else {
    //         println!("Condition value is: {condition_status}");
    //         continue;
    //     };
    // };
    // println!("loop_result value is: {loop_result}");

    // // loop labels to disambiguate between miltiple loops
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("1. count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("2. remaining = {remaining}");
    //         if remaining == 9 {
    //             println!("3. remaining is 9");
    //             break; // this break will end the loop in this scope.
    //         }
    //         if count == 2 {
    //             println!("4. count is 2");
    //             break 'counting_up; // this break will end the outer loop that has same label
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("5. End count = {count}");

    // // conditional loops with while
    // let mut number = String::new();
    // io::stdin()
    //     .read_line(&mut number)
    //     .expect("Enter only numeric");
    // let mut input_number: i32 = number.trim().parse().expect("Expected is a numeric");

    // while input_number != 0 {
    //     println!("number is not 0 but {input_number}");
    //     input_number -= 1;
    // }
    // println!("number become 0");

    // // looping through a collection with for
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 { // if the count is over than number of array item count then compiler will panic.
    //     println!("value on position {index} is {}", a[index]);
    //     index += 1;
    // }
    // // the same can be done using for, without needing to worry about array item count.
    // for element in a {
    //     println!("current element value is {element}");
    // }

    // for item in (1..5).rev() {
    //     println!("reverse looping - current position is {item}");
    // }

    // exercise-1: farenheit to celcius and convert it back to farenheit
    // c = (f - 32) * 5/9
    // f = (c * 9/5) + 32
    let mut faren = String::new();
    io::stdin()
        .read_line(&mut faren)
        .expect("expect tep in farenheit");
    let faren_numeric: f32 = faren.trim().parse().expect("enter temp in farenhei");
    let celcius_temp = f_to_c(faren_numeric);
    let faren_temp = c_to_f(celcius_temp);
    println!("Temp in celcius: {celcius_temp}");
    println!("Temp in farenhei: {faren_temp}");
}

fn f_to_c(faren: f32) -> f32 {
    (faren - 32.0) * 5.0 / 9.0 as f32
}

fn c_to_f(cel: f32) -> f32 {
    (cel * 9.0) / 5.0 + 32.0 as f32
}

// fn five() -> i32 {
//     5
// }

// fn plus_one(number: i32) -> i32 {
//     number + 1
// }

// fn call_me(name: &str, gender: char, age: u8) {
//     println!("Hello {name}, your gender is {gender} and your age is {age}.");
// }
