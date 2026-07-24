fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    shadowing();

}

//you can declare a new variable with the same name as a previous variable.
//Rustaceans say that the first variable is shadowed by the second, which means that the 
//second variable is what the compiler will see when you use the name of the variable. 
//In effect, the second variable overshadows the first, taking any uses of the variable name to itself 
//until either it itself is shadowed or the scope ends. We can shadow a variable by using the 
//same variable’s name and repeating the use of the let keyword as follows:
fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

//Shadowing is different from marking a variable as mut 
//because we’ll get a compile-time error if we accidentally try to 
//reassign to this variable without using the let keyword. 
//By using let, we can perform a few transformations on a value
//but have the variable be immutable after those transformations have completed.