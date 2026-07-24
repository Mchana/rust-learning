//each value in rust is a data type, we have scalar and compound
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    integer_literals();
    floating_point();
    operations();_
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