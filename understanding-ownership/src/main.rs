fn main() {
    code_to_break_things_up();
    return_multiple_values();
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
