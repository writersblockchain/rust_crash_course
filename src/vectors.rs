// vectors - resizable arrays 

use std::mem; 

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // re-assign value
    numbers[2] = 20; 

    // add onto vector 
    numbers.push(5);
    numbers.push(6); 

    println!("{:?}", numbers);

    //get single value 
    println!("Single value: {}", numbers[0]); 

    // get vector length 
    println!("Vector length: {}", numbers.len());

    // vectors are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3]; 
    println!("Slice: {:?}", slice); 

    // loop through vector values 
    for x in numbers.iter() {
        println!("Number: {}", x); 
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2; 
    }

    println!("Numbers Vec: {:?}", numbers); 



}