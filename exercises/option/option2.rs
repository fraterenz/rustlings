// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_value = Some(String::from("rustlings"));
    // destructure pattern, optional_value is Some(String) so using
    // let Some(value) the String inside optional_value will be moved
    // into value. This way you are doing a try catch without panicking
    if let Some(value) = optional_value {
        println!("the value of optional value is: {}", value);
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // while you can destructure, pop elements. See here for while let 
    // with Options:
    // https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html
    // Two Some(Some)) are needed since pop returns an Option and the elements
    // of optional_values_vec are Option too:
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop
    while let Some(Some(value)) = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
}
