fn main() {
    let mut cntr = 0;
    let dat_loop = loop {
        cntr += 1;
        if cntr == 1000 {
            break cntr/10;
        }
        println!("cycle : {cntr}");
    };
    println!("cntr : {dat_loop}");

}
