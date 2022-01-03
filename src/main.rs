use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 500] = [1; 500];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    println!("array occupies {:?} bytes", analyze_slice(&xs));

    analyze_slice(&ys[1 .. 4]);
}
