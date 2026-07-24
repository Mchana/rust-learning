fn main() {
    if_expression();
    if_else_expression();
    let_if();
    loop_example();

}

//Control flow - stuff like conditional expressions and loops

//if expressions
// Blocks of code associated with the conditions in if expressions are sometimes called arms
fn if_expression() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
} //It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error.

//Handling Multiple Conditions with else if
fn if_else_expression() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//Using if in a let Statement
//Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable

fn let_if() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    //the values that have the potential to be results from each arm of the if must be the same type
    //If the types are mismatched, as in the following example, we’ll get an error:

    //let number = if condition { 5 } else { "six" };
}

//Repetition with Loops:
//Rust has loop, while and for

//Repeating Code with loop

fn loop_example() {
    loop {
        println!("again!");
    }
} //this will run until we CTRL+C