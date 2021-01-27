// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)


fn main() {
    // note that you dont need a mut vec here because
    //
    // https://github.com/rust-lang/rustlings/issues/631
    let mut vec0 = Vec::<i32>::new();

    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    // this is wrong since borrowing mutably and thus vec0 has been invalidated
    // value borrowed after move error
    // println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    // borrowing mutably
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}
