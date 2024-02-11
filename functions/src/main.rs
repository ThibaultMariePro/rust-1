fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
    expression();
    let f = five();
    println!("the value of the five function : {f}");
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
