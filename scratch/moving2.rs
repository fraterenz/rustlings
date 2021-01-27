fn main() {
    let s1 = String::from("hello");
    // borrowing happens since s1 has not implemented the
    // Copy trait (is it on the Heap). s in `calculate_length`
    // will take the ownership of the data pointed by `s1` 
    // invalidating the pointer `s1`.
    let (s2, len) = calculate_length(s1);
    // this is wrong since `s1` has been invalidated!
    // println!("The length of '{}' is {}.", s1, len);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    // this is wrong since you cannot borrow as mutable
    // unless you use a mutable reference `&mut`.
    // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/mut.html
    // s.push('W');
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
