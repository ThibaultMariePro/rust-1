fn main() {
    let mut x = 5;
    println!("x : {x}");
    x = 6;
    println!("x : {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("x : {THREE_HOURS_IN_SECONDS}");

    let y = 3;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y in inner scope : {y}");
    }
    println!("y : {y}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces : {spaces}");

    let a = 2.0; // f64
    let b: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    
    let t = true;
    let f: bool = false; // with explicit type annotation
    
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (1245, 2.52, 99);
    let (tupx, tupy, tpuz) = tup;
    let elt = tup.1;
    println!("tupx : {tupx}");
    println!("second tup elt : {elt}");

    let arr: [i32; 6] = [1,2,3,4,5,99];
    
    let first_value_of_arr = arr[0];
    println!("{first_value_of_arr}");

    let five_thress_array = [3;5]; // same as [3,3,3,3,3]

} 

