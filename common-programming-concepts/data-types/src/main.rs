//each value in rust is a data type, we have scalar and compound
fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");
    integer_literals();
    floating_point();
    operations();
    boolean();
    character();
    tuples();
    tuple_pattern_matching();
    accessing_tuples();
    //Keep in mind that Rust is a statically typed language, 
    //which means that it must know the types of all variables at compile time.
    //rust can infer variable types in mayn cases, but sometimes like above, it needs declaring
}

//Scalar types:
//A scalar type represents a single value.
//Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

//Integers
//An integer is a number without a fractional component
//Each variant can be either signed or unsigned and has an explicit size. 
//Signed and unsigned refer to whether it’s possible for the number to be negative
//isize and usize work at the size of the architecure, either 32 or 64 bit depending on the system and os

//integer overflow occurs if a number is larger than the range the type can hold
//in debug mode, the propram will "panic" and exit with an error
// in --release, the value will "wrap around"- it will overflow and go back around to 1 (this is considered an error - don't do it)

fn integer_literals() {
    //signed
    let _x:i32 = -87;

    //unsigned
    let _y:i32 = 43;

    //decimal (represented by _ in integer literals - . is used in floating point)
    let _z:i32 = 98_222;

    //hex
    let _q:i32 = 0xff;

    //octal
    let _w:i32 = 0o77;

    //binary
    let _e:i32 = 0b1111_0000;

}

//Rust also has two primitive types for floating-point numbers, 
//which are numbers with decimal points. Rust’s floating-point types are f32 and f64
//Rust defaults to f64 

fn floating_point(){
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32
}

fn operations(){

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

}

//booleans
fn boolean(){
    let _t = true;

    let _f: bool = false; // with explicit type annotation    
}

//char is specified with single quotes
// Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. 
//Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust
fn character(){
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';    
}

//Compound types:
//2 types, tuples and arrays

//A tuple is a general way of grouping together a number of values 
//with a variety of types into one compound type. 
//Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
fn tuples(){
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
}

//The variable tup binds to the entire tuple because a 
//tuple is considered a single compound element. 
//to get the individual values out of a tuple, we can use pattern matching to destructure a tuple value,

fn tuple_pattern_matching() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    //This program first creates a tuple and binds it to the variable tup. 
    //It then uses a pattern with let to take tup and turn it into 
    //three separate variables, x, y, and z. This is called destructuring 
    //because it breaks the single tuple into three parts
}

//We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
fn accessing_tuples() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    //The tuple without any values has a special name, unit. 
    //This value and its corresponding type are both written () and represent an empty value or an empty return type
}