pub fn run() {
    let name = "Sean";
    let mut age = 27; 
    println!("My name is {} and I am {}", name, age);
    age = 28; 
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001; 
    println!("ID: {}", ID); 

    // Assign multiple variables 
    let (my_name, my_age) = ("Sean", 27); 
    println!("{} is {}", my_name, my_age); 
}