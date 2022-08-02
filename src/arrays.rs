use std::mem; 

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // re-assign value
    numbers[2] = 20; 

    println!("{:?}", numbers);

    //get single value 
    println!("Single value: {}", numbers[0]); 

    // get array length 
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3]; 
    println!("Slice: {:?}", slice); 


}