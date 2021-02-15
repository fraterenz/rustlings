// vec1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute the command `rustlings hint collections1` if you need hints.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // note that &a[..].to_vec() wont work since the
    // function calls have the precedence over the 
    // operator `&`! So it will first convert a into
    // a vec and then take the slice of it! Thanks 
    // reddit: https://www.reddit.com/r/learnrust/comments/lekdt1/how_come_these_two_very_similar_snippets_behave/gmekkox?utm_source=share&utm_medium=web2x&context=3
    // see here: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
    let v = &a[..];
    // equivalent to this because of deref coercion
    // let v = a.to_vec();

    (a, v.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
