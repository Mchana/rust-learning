fn main() {
    struct_instance_example();
    update_user();
    tuple_structs_example();
}

//structs are a data type that lets you package together and name multiple related values that
//make up a group. It's kinda like an object in OOP

// --Defining and instantiating structs

//structs are like tuples in that they can consist of different data types
//unlike in tuples, we name each piece of data so it's clear what those values mean
//that means those values don't have to be accessed in order
//do define it we enter the keyword "struct", then inside define the 
//names and types of pieces of data, called fields

#[allow(dead_code)] 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//to use that struct, we create an instance of that struct by specifying concrete values for
//each of those fields
//we don't have to specify the fields in order either

//example of instance:

fn struct_instance_example(){
    let mut user1 = User { //changed to mut to demonstrate dot notation
        active: true,
        username: String::from("somesuername1234"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    }; //to get a specific value from a struct, we use dot notation
      //if the instance is mutable, we can change the value using dot notation
      //and assigning it to a particular field

      user1.email = String::from("anotheremail@example.com")
}

//if we want to change fields, the entire instance must be mutable

//we can also pass fields into functions like this

#[allow(dead_code)] 
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// --Field init shorthand

//we also don't have to repeat each name
// because the parameter names and struct field names are the same, we can 
//use the field init shorthand to rewrite the above so it behaves the same,/
//but without the repetition

#[allow(dead_code)] 
fn build_user_init(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// --Creating instances with the Struct Update syntax

//sometimes we want to reuse a struct with most of the values of another instace of the same
//type but chanes some of them
//we can do this using the struct update syntax

fn update_user() {
    let user1 = User { //changed to mut to demonstrate dot notation
    active: true,
    username: String::from("somesuername1234"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
    };

    let user2 = User {
      active: user1.active,
      username: String::from("username"),
      email: user1.email,
      sign_in_count: user1.sign_in_count,
    };
    //we can also do this with less code
    let _user3 = User {
        username: String::from("usergnome"),
        ..user2 //..user2 means that fields not set should have the same value as user2
                //this must come last
    };
}

//the struct update syntax uses = as an assignment, because it moves the data
//we can no longer use user 2 after creating user3 as the string of email was moved into user3
//active and sign in are types that implement the copy trait, so we could still use those
//we can also use user2.email as it hasn't been moved out of user2

// -- Creating different types with tuple structs

//Rust supports structs that look similar to tuples, called "tuple structs" 
//this means they have the added meaning the struct name provides, but don't have names
//associated with their fields - they just have the types
//This is useful when you want to give a whole tuple a name and make it different from other tuples
//and when naming each field would be verbose or redundant

fn tuple_structs_example() {
    #[allow(dead_code)] 
    struct _Colour(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = _Colour(0,0,0,);
    let origin = Point(0,0,0);

    let Point(_x,_y,_z) = origin;
    //used to destructure the values in origin into variables named x y and z
}

//the black and origin fields are different types because they're instances of different
//tuple structs. Each struct you define is it's own type, even though the fields within may
//have the same type
//for example a function that takes the parameter of type Colour cannot take Point as an argument
//even thought they're made up of the same values
//they're similar to tuples in that they can be destructured into individual pieces
//and you can use a. to access the indivial values. Unlike tuples, tuple structs require 
//you to name the type of struct when you destructure them


// -- Defining unit-like structs

