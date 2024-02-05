use std::io;

fn main() {
    let a = [10,20,30,40,50];
    println!("Please enter an array index:");

    let mut idx = String::new();

    io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");
    
    let idx: usize = idx
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[idx];

    println!("At index {idx}, value in array is : {element}");
}
