fn main() {
    let vec0 = Vec::new();

    // moving the value returned by fill_vec to vec1. 
    // Since fill_vec moves vec0 to vec and goes out of
    // scope, vec0 is not available anymore (but the data
    // associated to vec0 it is still avaiable since it has
    // at least one owner, that is not ALL owners pointing at 
    // the vector had been dropped)
    let mut vec1 = fill_vec(vec0);
    vec1.push(21);
    // this is wrong since vec0 is not available anymore
    // vec0.push(23);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // moving vec0 to vec; vec0 is an invalidated reference,
    // that is a pointer that is no longer valid since vec
    // took the ownership of vec0 (but the data associated 
    // to vec0 is still present since vec has taken its ownership,
    // that is there is at least 1 pointer referring to it)
    let mut vec = vec;
    vec.push(2);
    // vec goes out of scope and it's dropped: that data
    // associated to vec is still valid, but vec is a invalidated
    // reference (pointer no longer valid)
    vec
}
