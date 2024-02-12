fn main() {
    let mut cntr = 0;
    let dat_loop = loop {
        cntr += 1;
        if cntr == 1000 {
            break cntr / 10;
        }
        println!("cycle : {cntr}");
    };
    println!("cntr : {dat_loop}");

    'loop1: loop {
        println!("entered loop1");
        'inner: loop {
            println!("entered inner loop");
            break 'loop1;
        }
        println!("this will never be reached");
    }
    println!("exited loop1 without reaching previous println!");

    let mut num = 5;
    while num != 0 {
        println!("{num} turn remaining");
        num -= 1;
    }
    println!("while loop exited");

    let fora = [10, 20, 30, 40, 50];
    let mut idx = 0;
    while idx < 5 {
        println!("printing array value : {}", fora[idx]);
        idx += 1;
    }
    for elt in fora {
        println!("printing same thing with for loop : {elt}");
    }

    for n in (0..5) {
        println!("{n}");
    }
}
