fn main() {
    calc_rectangle();

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
}

fn calc_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        " the area of the rectangle is {} square pixels.",
        rect1.area()
    );
}