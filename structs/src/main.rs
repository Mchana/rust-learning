fn main() {
    struct_instance_example();
}

//structs are a data type that lets you package together and name multiple related values that
//make up a group. It's kinda like an object in OOP

// --Defining and instantiating structs

//structs are like tuples in that they can consist of different data types
//unlike in tuples, we name each piece of data so it's clear what those values mean
//that means those values don't have to be accessed in order
//do define it we enter the keyword "struct", then inside define the 
//names and types of pieces of data, called fields

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

fn build_user_init(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}