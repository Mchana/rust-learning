fn main() {
    code_to_break_things_up();
    ownership_demonstration();
    return_multiple_values();
    string_example();
    string_example_mutation();
    memory_example();
    interacting_with_move();
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

//References and Borrowing
