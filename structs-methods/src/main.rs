use std::u32;

fn main() {
    calc_rectangle();
    rectangle_with_impl_struct_parameter();
    rectangle_can_hold();
}

//Methods are similar to functions: 

//We declare them with the fn keyword and a name, 
//they can have parameters and a return value, 
//and they contain some code that’s run when the method is called from somewhere else. 
//Unlike functions, methods are defined within the context of a struct 
//(or an enum or a trait object, which we cover in Chapter 6 and Chapter 18, respectively), 
//and their first parameter is always self, 
//which represents the instance of the struct the method is being called on.

// --Methods Syntax

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn calc_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "the area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

//to define the function within the context of Rectangle, we start an impl(implementation)
//block for Rectangle. 
//Everthing within the impl block will be associated with teh Rectangle type
//Then we move the area function within the impl curly brackets, and change the first
//(and only) parameter to be &self in the signature and everywhere within the body
//the main function (calc_rectangle), where we called the area function and passed rect1 as 
//an arguement in /structs, we can instead use the method syntax to call the area method 
//on our Rectangle instance
// the method syntax goes after an instance - we add a dot followed by 
//the method name, parentheses and arguments

//in the signature for area, we use &self instead of rectangle: &Rectangle
//the &self is actually short for self: &Self 
//within an impl block, the type Self is an alias fot the type that the impl block is for
//Methods must have a parameter named self of type Self for their first parameter,
//so Rust lets you abbreviate that in the first parameter spot.
//Note that we still need to use the & in front of the self shorthand to indicate that this method
//only borrows the Self instance, just as we did in rectangle: &Rectangle
//Methods can take ownership of self, borrow self immmutably, or as we've done here,
//borrow self mutably, just as they can any other parameter

//we chose &self here for the same reason we used &Rectangle in the function version:
//We don't want to take ownership, and we just want to read data from the struct - not write to it
//if we wanted to change the instance that we've called on the methods as part of what the method
//does, we'd use &mut self as the first parameter
//having a method that takes ownership of the instance by just using self as the first
//parameter is rare, this technique is usually used when the method transforms self
//into something else, and you want to prevent the caller from using the original instance
//after the transformation

//the main reason for using methods instead of functions, in addition to providing method syntax
//and not having to repeat the type of self in every method's signatire, is for organisation
//we've put all the things we can do with an instance of a type in one imply block rather
//than make future users of our code search for the capabilites of Rectangle in various
//places in in the library we provide

//note that we can choose to give a method the same name as one of the struct's fields
//for example, we can define a method on Rectangle that is also named width

#[derive(Debug)]
struct Rectangle2{
    width: u32,
    #[allow(dead_code)]
    height: u32,
}

impl Rectangle2 {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn rectangle_with_impl_struct_parameter() {
    let rect2 = Rectangle2 {
        width: 30,
        height: 50,
    };
    if rect2.width() {
        println!(
            "the rectangle has a nonzero width; it is {}",
            rect2.width
        )
    }
}

//here we're choosing to make the width method return true if the value in the instance's width 
//field is greater than 0 and false if the value is 0:
//we can use a field within a method of the same name for any purpose.
//in main when we follow rect1.width with parentheses, Rust knows we mean the field width

//often, but not always, when we give a method the same name as a field we want it to only 
//return the value in the field and do nothing else
//Methods like this are called getters, and rust does not implement them automatically
//for struct fields like other language do
//Getters are useful because you can make the field private but the method public,
//and thus enable read only access to that field as part of the type's public API

// --Where's the -> Operator??

//In C and C++, two different operators are used for calling methods: 
//You use . if you’re calling a method on the object directly 
//and -> if you’re calling the method on a pointer to the object 
//and need to dereference the pointer first. 
//In other words, if object is a pointer, object->something() is similar 
//to (*object).something().

//Rust doesn’t have an equivalent to the -> operator; 
//instead, Rust has a feature called automatic referencing and dereferencing. 
//Calling methods is one of the few places in Rust with this behavior.

//Here’s how it works: When you call a method with object.something(), 
//Rust automatically adds in &, &mut, or * so that object matches the signature of the method.
// In other words, the following are the same:

/*
p1.distance(&p2);
(&p1).distance(&p2);
 */

 //The first one looks much cleaner. 
 //This automatic referencing behavior works because 
 //methods have a clear receiver—the type of self.
 // Given the receiver and name of a method, Rust can figure out definitively 
 //whether the method is reading (&self), mutating (&mut self), or consuming (self).
 // The fact that Rust makes borrowing implicit for method receivers is a big part 
 //of making ownership ergonomic in practice.

 // --Methods with more parameters

//we can add multiple methods on structs, for example for this program

//This time we want an instance of Rectangle to take another instance of Rectangle and 
//return true if the second Rectangle can fit completely within self (the first Rectangle); 
//otherwise, it should return false.

fn rectangle_can_hold() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
        let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
        let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

     println!(
    "can rect1 hold rect2? {}", 
    rect1.can_hold(&rect2));
        println!(
    "can rect1 hold rect3? {}", 
    rect1.can_hold(&rect3)); 
}

//we would expect this for the output
//Can rect1 hold rect2? true
//Can rect1 hold rect3? false

// we know we want to define a methods, so it will be within the impl Rectangle block
//this method name will be can_hold, and will take an immutable borrow of another Rectangle
//as a parameter
//we can tell what type the parameter will be by looking at the code that calls the method:
//rect1.can_hold(&rect2) passes in &rect2, which is an immutable borrow to rect2
//because we only read to read rect2(rather than a write, which would use an immutable borrow)
//and we want the rectangle_can_hold() to maintain ownership of rect2 so we can use it again
//after calling the can_hold method
//the return value of can_hold will be a Boolean, and the implementation will check whether
//the width and height of self are greater than the width and height of the other Rectangle,
//respectively

//this code is added to the above impl block, it's here too just for continuity

/*
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
*/

//now when we run this code with "cargo run", we should get the desired output