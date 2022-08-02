pub fn run() {
    // default is "i32"
    let x = 1; 

    //default is "f64"
    let y = 2.5; 

    // add explicit type
    let z: i64 = 454445454545; 

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max is i64: {}", std::i64::MAX);

    // boolean
    let is_active: bool = true; 

    // get boolean from exoression
    let is_greater: bool = 10 > 5; 

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, x, is_active, is_greater, a1, face));
}