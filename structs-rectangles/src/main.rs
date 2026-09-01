fn main(){
    rectangles_no_tuples();
    rectangles_refactored_with_tuples();
    rectangles_refactored_with_structs();
    rectangles_with_derived_traits();
    rectangles_with_dbg();
}

fn rectangles_no_tuples() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "the area of the rectangle is {} square pixels,",
        area(width1, height1)
    );

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//this calculates the area of a rectangle, but the funtion we wrote has 2 parameters,
//and it's unclear if the parameters are related
//it would be much more readable to group width and height together

// --refactoring with tuples

fn rectangles_refactored_with_tuples() {
    let rect1 = (30, 50);
    print!(
        "the area of the rectangle is {} square pixels,",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//This lets us add some structure, now that we're just passing 1 argument
//but tuples don't name the element, so we have to index parts of the tuple in area2
//making the calculation less obvious
//we have to keep in mind that index 0 width and height is 1
//this isn't obvious to anyone reading or using the code, which can introduce errors

// -- Refactoring with structs

//we use structs to add meaning to the data - we can transform the tuple we're using into a 
//struct with a name for the whole as well as names for the parts

struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangles_refactored_with_structs(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
        print!(
        "the area of the rectangle is {} square pixels,",
        area3(&rect1)
    );
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//Here, we’ve defined a struct and named it Rectangle. 
//Inside the curly brackets, we defined the fields as width and height, 
//both of which have type u32. 
//Then, in main, we created a particular instance of Rectangle 
//that has a width of 30 and a height of 50.

//Our area function is now defined with one parameter, 
//which we’ve named rectangle, 
//whose type is an immutable borrow of a struct Rectangle instance. 
//As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. 
//This way, main retains its ownership and can continue using rect1, 
//which is the reason we use the & in the function signature and where we call the function.

//The area function accesses the width and height fields 
//of the Rectangle instance (note that accessing fields of a borrowed 
//struct instance does not move the field values, 
//which is why you often see borrows of structs).
// Our function signature for area now says exactly what we mean: 
//Calculate the area of Rectangle, using its width and height fields. 
//This conveys that the width and height are related to each other,
// and it gives descriptive names to the values 
//rather than using the tuple index values of 0 and 1. This is a win for clarity.

// -- Adding functionality with derived traits

//we can't use the println! macro like we have done in other programs with the below

/*
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1}");
}

compiler says

error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
*/

//this is because with structs, they aren't primitive types - Rust doesn't know 
//what it should and shouldn't display
//but the compiler is beautiful and gives us a hint - 
/*
   |                        |`Rectangle` cannot be formatted with the default formatter
   |                        required by this formatting parameter
 */

//so we change the println! line to 
/*
    println!("rect1 is {rect1:?}");
 */

// "?" tells println! that we want to use an output format called Debug
//this enables us to print the struct in a way that's useful
//however we still get 

/*
error[E0277]: `Rectangle` doesn't implement `Debug`
   |                        required by this formatting parameter
   |

 */

 //to print this, we have to explicitly opt in to debug to make that available for out struct

 #[derive(Debug)]
 struct Rectangle2 {
    width: u32,
    height:u32,
 }

 fn rectangles_with_derived_traits() {
    let rect2 = Rectangle2 {
        height: 30,
        width: 50,
    };
    println!("rect2 is {rect2:#?}"); 
    //:? makes it use the debug format, 
    //:#? makes the print multi line and easier to read
 }

//there are also cases where we're interested in the value of width, as well as the struct
 
// --using dbg!

fn rectangles_with_dbg(){
    let scale = 2;
    let rect1 = Rectangle2 {
        width: dbg!(30*scale),
        height: 50,
    };
    dbg!(rect1);
}

//We can put dbg! around the expression 30 * scale and, 
//because dbg! returns ownership of the expression’s value, 
//the width field will get the same value as if we didn’t have the dbg! call there. 
//We don’t want dbg! to take ownership of rect1, 
//so we use a reference to rect1 in the next call.

//we can continue to refactor by turning this area function into a method