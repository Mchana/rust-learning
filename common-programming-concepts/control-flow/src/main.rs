fn main() {
    if_expression();
    if_else_expression();
    let_if();
    loop_example();
    loop_returning_value();
    loop_labels();
    while_loop();
    while_loop_with_for();
    while_loop_without_for();
    while_range_example();

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
        break
    }
} //this will run until we CTRL+C (the break is so i don't break the computer)

//Returning Values from Loops
//One of the uses of a loop is to retry an operation you know might fail, 
//such as checking whether a thread has completed its job. 
//You might also need to pass the result of that operation out of the 
//loop to the rest of your code. 
//To do this, you can add the value you want returned after the 
//break expression you use to stop the loop; that value will be r
//eturned out of the loop so that you can use it, as shown here:

fn loop_returning_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

//Disambiguating with Loop Labels
//If you have loops within loops, break and continue apply to the 
//innermost loop at that point. You can optionally specify a loop label 
//on a loop that you can then use with break or continue to specify that those keywords 
//apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

//Streamlining Conditional Loops with while
//saves us using loop with if, else and break which is a pain

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

//Looping Through a Collection with for
fn while_loop_without_for() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
//we can compact a while loop by using "for"
fn while_loop_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
} //this is safer than a regular while loop and a loop loop

//we can also use a Range 
//this example also uses .rev

fn while_range_example() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}