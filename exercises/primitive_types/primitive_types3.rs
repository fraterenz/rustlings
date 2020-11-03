// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    //let a: [u16; 100] = [10; 10]; this is wrong since the expected
    // size of a is 100 but got an array of 10 only elements
    //
    // this is wrong too since 
    // let a: [std::string::String; 100] = ["10".to_string(); 100];
    // the element is not type `Copy` since strings are stored on
    // the heap: 'the trait `std::marker::Copy` is not implemented for 
    // `std::string::String`'
    let a: [u8; 100] = [10; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
