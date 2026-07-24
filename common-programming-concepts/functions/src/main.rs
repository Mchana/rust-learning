//Rust code uses snake case as the conventional style for function and variable names, 
//in which all letters are lowercase and underscores separate words

fn main() {
    parameters();
    statements();
    expressions();
    five();
    func_with_return_value();
}
//Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller
//they can be before or after "main", we love a compiled language

//Parameters
//In function signatures, you must declare the type of each parameter. 
//This is a deliberate decision in Rust’s design: 
//Requiring type annotations in function definitions means the compiler 
//almost never needs you to use them elsewhere in the code to figure out what type you mean.
fn parameters() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");

//When defining multiple parameters, separate the parameter declarations with commas, like this:
    print_labeled_measurement(5, 'h');

    fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

}

//Statements and expressions:
//Statements are instructions that perform some action and do not return a value.
//Statements have a ;
//Expressions evaluate to a resultant value with no ;

fn statements() {
    let _y = 6;

    //Statements do not return values. Therefore, you can’t assign a let statement to another variable, 
    //as the following code tries to do; you’ll get an error:
    //let x = (let y = 6);

}

//Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

//Functions with Return Values
//Functions can return values to the code that calls them. 
//We don’t name return values, but we must declare their type after an arrow (->). 
//In Rust, the return value of the function is 
//synonymous with the value of the final expression in the block of the body of a function. 
//You can return early from a function by using the return keyword and specifying a value, 
//but most functions return the last expression implicitly. 

//this is perfectly valid - you can have a function that just returns a value by itself
//however, we must define the type here
//it has no ; becuase it an expression - we want  return value
fn five() -> i32 {
    5
}

fn func_with_return_value() {
    let x = five();

    println!("The value of x is: {x}");
}