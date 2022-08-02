pub fn run() {
    //print to  console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Sean", "Fresno, CA");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}", 
        "Sean", "Temecula, CA", "code."
    );

    //Named Arguments
    println!(
        "{name} likes to play {activity}", 
        name= "Sean",
        activity = "basketball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);


}