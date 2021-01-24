// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

fn main() {
    // need to modify and call it mut
    let mut vec0 = Vec::new();
    // mutable reference: borrowing, if not ref & would have been moving
    // vec1 did not take the owernship since it is a reference but 
    // modifies the variable since it is a mut reference
    fill_vec(&mut vec0);
    // let mut vec1 = &fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);
    // vec1.push(88);

    //println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

//fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
fn fill_vec(vec: &mut Vec<i32>) -> () {
    // creates a shallow copy + invalidates: moving
    //let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
    // but once returned, the owner of vec goes out of scope the object is 
    // dropped since in rust once the owner goes out of scope the object is
    // dropped!
    // vec
}
