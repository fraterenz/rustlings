// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode (the value
// that occurs most often; a hash map will be helpful here) of the list.
// solutions are here:
// https://codereview.stackexchange.com/questions/173338/calculate-mean-median-and-mode-in-rust
use std::collections::HashMap;

fn mean(v: &[f32]) -> f32 {
    v.iter().sum::<f32>() / v.len() as f32
}

fn median(v: &mut [f32]) -> f32 {
	// need to either create ref mut or copy
	v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());    
	let idx = v.len() as u16 / 2 ;
	v[idx as usize]
}

fn main() {
    let mut v1 = [2., 2., 2.];
    let mut v2 = [2., 1., 5., 3.];
    let mut v3 = [2., 5., 3.];
    println!("{} is the mean of {:?}", mean(&v1), v1);
    println!("{} is the median of {:?}", median(&mut v1), v1);
    println!("{} is the mean of {:?}", mean(&v2), v2);
    println!("{} is the median of {:?}", median(&mut v2), v2);
    println!("{} is the mean of {:?}", mean(&v3), v3);
    println!("{} is the median of {:?}", median(&mut v3), v3);
}
