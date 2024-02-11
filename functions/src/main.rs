fn main() {
    another_function(5, 'h');

    expression();
    
    let f = five();
    println!("the value of the five function : {f}");

    conditions_test();
}

fn another_function(x: i32, y: char){
    println!("An other function has been executed {x}, {y}");
}

fn expression(){
    let x = {
        let y = 3;
        y + 1
    };
    println!("value in inner block: {x}");
}

fn five() -> i32{
    5
}

fn conditions_test(){
    let cndt = true;
    let output = if cndt {5} else {6};
    println!("regarding condition, value of output is: {output}");
}
