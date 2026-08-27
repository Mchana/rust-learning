fn main() {
    code_to_break_things_up();
    ownership_demonstration();
    return_multiple_values();
    string_example();
    string_example_mutation();
    memory_example();
    interacting_with_move();
    scope_and_assignment();
    fun_with_clone();
    copying_stack_only_data();
    ownership_and_functions();
    return_values_and_scope();
    references_and_borrowing();
    mutable_references();
    multiple_mut_references();
    mutable_and_immutable_refs();
    dangle();
    uses_first_word();
}

//Ownership
//Rust doesn't have a garbage collector 
//Memory is managed with "ownership" with a set of rules the compiler checs
//if the rules are violated, it won't compile
//this is a balance between the manual memory management of other language like C and a garbage collector

//Heap and Stack

// The stack stores values in the order it gets them and removes the values in the opposite order. 
//This is referred to as last in, first out (LIFO).

//The heap is less organized: When you put data on the heap, you request a certain amount of space.
// The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, 
//and returns a pointer, which is the address of that location. This process is called allocating on the heap

//Pushing to the stack is faster than allocating on the heap because the allocator 
//never has to search for a place to store new data; that location is always at the top of the stack. 
//Comparatively, allocating space on the heap requires more work because the allocator must first find a 
//big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

//Accessing data in the heap is generally slower than accessing data on the 
//stack because you have to follow a pointer to get there. 
//Contemporary processors are faster if they jump around less in memory

fn code_to_break_things_up() -> i32{
    5
    //serves no purpose, just to break up the notes lol
}

//Ownership Rules
//-->Each value in Rust has an owner.
//-->There can only be one owner at a time.
//-->When the owner goes out of scope, the value will be dropped.

fn ownership_demonstration()
    {                      // s is not valid here, since it's not yet declared
        let _s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

//In other words, there are two important points in time here:

//-->When s comes into scope, it is valid.
//-->It remains valid until it goes out of scope.


//String Type
//To demonstrate the rules of ownership, we need a more complex data type 
//the types previously discussed are of known size, and can be popped on and off the stack when their scope is over
//the String type is stored on the heap so is best to demonstrate Ownership

//We've used string literals before, but they aren't suitable for every situation involving text
//they're immutable , and not every string value can be known when we write our code, e.g. user input
//this type manages data allocated on the heap

fn string_example(){
    let _s =String::from("hello");
    //:: allows us to namespace this from without using string_from
    //ngl no idea what that means yet 
}

fn string_example_mutation(){
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`

} //why can string be mutated but literals cannot? it's because of how these types deal with memory

//--Memory and allocation

//with string literals, we know the contents at compile time, so the text is hardcoded directly into the final exe
//this doesn't work with unknown text that might change size

//With the String type, in order to support a mutable, growable piece of text, 
//we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

//--The memory must be requested from the memory allocator at runtime.
//--We need a way of returning this memory to the allocator when we’re done with our String.

//when we call String::from the implementation requests the memory it needs
// Unlike other languages with no garbage collectors, Rust automatically returns memory when it goes out of scope

fn memory_example(){
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no longer valid

//at this point, Rust calls a special function for us called 'drop' to return the memory
//it does this on  a }

//Variables and Data Interacting with Move:

//Multiple variables can interact with the same data in different ways in Rust. 
//for example, an integet and a string

fn interacting_with_move(){
    let x = 5;
    let _y = x;

    let s1 = String::from("hello");
    let _s2 = s1;

//we would assume these work in different ways, however they are different under the hood
}

//a String is made of 3 parts - a pointer, length and capacity
//the pointer points to where the String is stored on the heap in memory
//the length is how much memory, in bytes the contents of String is using
//the capacity is how much memory in bytes it has been allocated
// for example "hello" would point to the memory, with a length of 5 and a capacity of 5
// in memory it would have an index (1,2,3,4,5) representing (h,e,l,l,o) in the heap
// when we assign s1 to s2, the String is copied, meaning we copy the pointer, length and capacity
//the pointer still points to the same place in the heap, it does not copy the data
//this saves runtime if the data is large

//as said earlier, when a vairable goes out of scope, Rust calls a "drop" function
//if both s1 and s2 point to the same location, this causes a problem
//if both go out of scope, they will try and free the same memory
//this is a "double free" error, a memory safety bug that can cause memory corruption and 
//security vulnerabilities
//hence after s2=s1, Rust considers s1 no longer vaild, and doesn't need to free anything

//this could be considered a "shallow copy" in other languages, because it copies the 
//pointer, length and capacity without copying the data
//however, because Rust invalidates s1, it is actually a "move"
//Rust will NEVER automatically create a "deep" copy
//hence any automatic copying will be inexpensive in terms of performance


//the inverse is true for the relationship between scoping, ownership and memory
//being freed via the "drop" function
//when you assign a completely new value to an existing variable, Rust will
//call drop and free the existing memory immediately
fn scope_and_assignment() {
    #[allow(unused_assignments)]
    let mut s = String::from("hello");
    s = String::from("ahoy");
    print!("{s}, world!");
} //here, "hello" is immediately replaced with "ahoy", so the print will be "ahoy, world!"

// --Cloning data

//if we do want to do a deep copy of the data, we can use clone

fn fun_with_clone(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    print!("s1 = {s1}, s2 = {s2}");
} //this explicitly copies the heap data

// -- Copying stack only data

fn copying_stack_only_data(){
    let x = 5;
    let y = x;
    print!("x = {x}, y = {y}");
}

//this seems to contradict the above - we didn't clone x, but it is still valid and 
//wasn't moved into y
//this is because types such as integers have a known size at compile time,
//and so are copied to the stack rather than the heap, so copies are quick to make
//there's no reason we would want to prevent x being valid after y in this case,
//meaning there's no difference between deep and shallow copying. Calling "clone"
//wouldn't do anything different from a shallow copy, so we leave it out

//Rust has a special annotation called the "copy" trait that can be placed on types stored
//on the stack, such as integers. if a type implements the "copy" trait, variables that use it
//do not move, but rather are trivially copied, making them still valid after assignment 
//to another variable

//Rust won't let us annotate a type with the "copy" trait if the type, or any of it's parts
//has implemented the "drop" trait
//if the type needs something special to happen when the value goes out of scope and we 
//add the "copy" trait, we'll get a compile time error

//as a general rule, any group of simple scalar values can implement "copy",
//and and nothing that requires allocation or is some form of resource can implement "copy"
//some types that use copy include:
// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating-point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy.
//  For example, (i32, i32) implements Copy, but (i32, String) does not.

// -- Ownership and functions
//the mechanics of passing a value to a function are similar to to those as when assigning a 
//value to a variable
//passing a variable to a function will move or copy, just as assignment does

//this is an example of variables going in and out of scope:
fn ownership_and_functions(){
    let s = String::from("hello"); //s comes into scope
    takes_ownership(s); // s's vaslue moves into the function
                        //and so is no longer valid here
    let x = 5;          //x comes into scope
    makes_copy(x);      //because i32 implements the copy trait
                       // x does NOT move into the function
                        //so it's ok to use x afterwards
} //here x goes out of scope, then s. because s's value was moved, nothing special happens

fn takes_ownership(some_string: String) { //some_string comes into scope
    print!("{some_string}");
} //some_string goes out of scope and "drop" is called, the backing memory is freed

fn makes_copy(some_integer: i32) { //some_integer comes into scope
    print!("{some_integer}");
} //some_integer goes out of scope. Nothing special happens

//if we tried to use s after the call to takes_ownership(), Rust would throw a compile time
//error. These static checks prevent us from mistakes. 

// -- Return values and scoe

//Return values can also transfer ownership

fn return_values_and_scope(){
    let _s1 = gives_ownership(); //gives_ownership moves its return value into s1
    let s2 = String::from("hello"); //s2 comes into scope
    let _s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back, which also
                                       //moves it's return value into s3
} //here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1
//goes out of scope and is dropped
fn gives_ownership() -> String { //gives_ownership will move its return value into the funtion
                                 //that calls it
    let some_string = String::from("yours"); //some_string comes into scope
    some_string //some_string is returned and moves out to the calling function
}
//this function takes a string and returns a string
fn takes_and_gives_back(a_string: String) -> String { //a_string comes into scope
    a_string //is returned and moves out to the calling function
}

//the ownership of a variable follows the same pattern every thing: assigning a value
// to another variable moves it. When a variable that includes data on the heap moves
//out of scope, the value will be cleaned up by "drop" unless ownership of the 
//data has been moved to another variable

//while this works, taking ownership and then returning ownership with every function
//is a bit tedious. What if we want to let a funcion use a value but not take ownership?
//we can solve this tedium using a tuple

//returning multiple values using a tuple
fn return_multiple_values(){
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("the length of '{s2}' is {len}.");
}

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len(); //.len returns length of a string
    (s, length)
}

//the above is kind of a pain and there's a better way to do it

// --References and Borrowing
//this is a way to use a value without transferring ownership - references
//the issue with the above tuple code is that we have to return the String to the calling
//function so that we can still use the String after the call to calculate_length, becauase
//the String was moved into that function

//instead, we can provide a reference to the String value
//a reference is like a pointer, in that in that it's an address we can follow to access data 
//stored at that address, that data is owened by by some other variable
//unlike a pointer, a reference is guaranteed to point to a particular value
//for the life of that reference


//here is a way we can define and use a calculate_length function that has a reference
//to an object as a parameter instead of taking ownership of the value

fn references_and_borrowing(){
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length2(s: &String) ->usize { //s is a reference to the String
    s.len()
} //here s goes out of scope, but because s does not have ownership, it is not dropped

//the tuple code in the variable declaration and the function return is gone
// we pass &s1 into calculate_length2, and in it's definition we take &String rather than String
// & represents references, and they allow you to refere to a value without taking ownership

//the opposite of referencing is dereferencing, using the * operator which we'll explore later

//when functions have references instead of parameters, we don't need to return the value
//because we never had ownership
//we call the action of reference "borrowing" as when we're done with it, we give it back

//what if we try to modify something we're borrowing? it doesn't work

/*
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

// -- Mutable references

fn mutable_references() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(" , world");
}

//to make a mutable reference, first we change s to be mut
//then we create a mutable reference with &mut s, and update the function signature to accept
//a mutable reference. This makes it very clear that the change() function will mutate the
//value it borrows

//mutable references have one big restriction - if you have a mut references to that value,
// you can have no other references to that value. This code that attempts to create 2
//mut references to s will fail

/*
let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");

$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{r1}, {r2}");
  |                -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
 */

//this prevents "data races" at compile time, which heppens when 3 conditions occur:
//-2 or more pointers are used to access data at the same time
//-at least one of the pointers is being used to write to the data
//- there's no mechanism being used to syncronise access to the data

//data races cause undefined behaviour adn can be difficult to diagnose and ficx
//when you're trying to track them down at runtime
//Rust prevents this by refusing to compile code with multiple data races

fn multiple_mut_references() {
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;

    } //r1 goes out of scope here, so we can make a new reference here with no problems
    let _r2 = &mut s;
}
//as we can see, Rust allowed for multiple references, just not simultaneous ones

//Rust also has a similar rule for combining mutable and immutable references

/*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");

$ cargo run
Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
--> src/main.rs:6:14
|
4 |     let r1 = &s; // no problem
|              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
|              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{r1}, {r2}, and {r3}");
|                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

 */

 //we also cannot have a mutable reference while we have an immutable references to the same value

//we can have multiple immutable references because they don't change the value of data

//for example, this will compile because it has the last use of immutable references  
//and the mutablea ref is after the println!

fn mutable_and_immutable_refs() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    //r1 and r2 won't be used after this point
    let r3 = &mut s;
    print!("{r3}");
}

// --Dangling references

//in languages with pointers, it is easy to create a dangling pointer - a pointer that
//references a location in memory that may have been given to someone else
//rust prevents these with compile time errors

/*
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

    $ cargo run
Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
--> src/main.rs:5:16
|
5 | fn dangle() -> &String {
|                ^ expected named lifetime parameter
|
= help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
|
5 | fn dangle() -> &'static String {
|                 +++++++
help: instead, you are more likely to want to return an owned value
|
5 - fn dangle() -> &String {
5 + fn dangle() -> String {
|

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

 */

//in this code, because s is create inside dangle(), when the code of dangle() is finished,
//s will be deallocated. but we tried to return a reference to it - this means the ref will
//point to an invalid String. Rust won't let us do this
//the solution here is to return the String directly

fn dangle() -> String {
    let s = String::from("hello");
    s
}

//this works as ownership is moved out, and nothing is deallocated

//rules of references:
//-at any time you can have either one mutable reference or any number of immutable refs
//references must always be valid

// --Slices

//Slices let you reference a contiguous series of elements in a collection. A slice is a kind
//of reference, so it doesn not have ownership

//Here’s a small programming problem: 
// Write a function that takes a string of words separated by spaces 
// and returns the first word it finds in that string. 
// If the function doesn’t find a space in the string, 
// the whole string must be one word, so the entire string should be returned.

//let's try without slices first:

//the first_word() function has a parameter type of &String. We don't need ownership so this is fine
//(in idiometic rust,  functions don't take ownership unless they need to)
//but what do return? we don't have a way to to talk about part of a string. However, we
//could return the index of the end of the word, indicated by a space

fn first_words(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
        s.len()
}

//because we need to go through the String element by elements and check if the value
//is a space, we convert our string to an array of bytes using the as_bytes() method

//next we create an interator of the array of bytes using the iter() method.
//iter returns each each elements in a collection adn that "enumerate" wea[s the result
//of iter() and results each element as part of a tuple instead
//the first elements of the tuple returned from enumerate is teh index, and the second
//element is a reference to the element - this is more convinient than calculating the index ourselves

//because the enumerate method returns a tuple, we can use patterns to destructure that tuple
// In the for loop, we specify a pattern that has i for the index in the tuple 
//and &item for the single byte in the tuple. 
//Because we get a reference to the element from .iter().enumerate(), 
//we use & in the pattern.
//Inside the for loop, we search for the byte that represents the space by
//using the byte literal syntax. If we find a space, we return the position.
//Otherwise, we return the length of the string by using s.len().

//We now have a way to find out the index of the end of the first word in the string, 
//but there’s a problem. We’re returning a usize on its own,
// but it’s only a meaningful number in the context of the &String.
// In other words, because it’s a separate value from the String, 
//there’s no guarantee that it will still be valid in the future. 

//consider this program that uses first_word()

fn uses_first_word() {
    let mut s = String::from("hello world");
    let _word = first_words(&s); //word will get the value 5
    s.clear(); //this empties the String, making it equal to ""

    //word still has the value 5 here, but s no longer has any content that we could 
    //meaningfully use here with the value 5, so word is now totally invalid
}