// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    // this works because a slice implements the Deref trait
    // and thus Defer coercion is used and we dont need to
    // write &[2, 3, 4] or *nice_slice
    assert_eq!([2, 3, 4], nice_slice)
}
