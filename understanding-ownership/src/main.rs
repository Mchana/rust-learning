fn main() {
    return_multiple_values();
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
