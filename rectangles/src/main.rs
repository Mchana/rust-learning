fn main(){
    rectangles_no_tuples();
    rectangles_refactored_with_tuples();
    rectangles_refactored_with_structs();
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

