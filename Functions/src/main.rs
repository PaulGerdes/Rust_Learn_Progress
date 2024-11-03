fn main() {
//  Parameter in Rust functions:
    another_function(5);
//  Another Example:
    print_labeled_measurement(5, 'h');

/*
    Statements and Expressions
        Statements perform some action without return
        Expression perform some action too but have a return

        When you create a Variable with let your already creating Statements*/
        let _x = 6;
//      so you cant assign a let statement to another variable like this:
//      let x = (let y=6);    This would create a ERROR

//      With Curly Braces you can create Expression in Variables in Rust:
        let y = {
            let x =3;   //these both
            x+1             // are the Expression
        };
        println!("{y}");

/*
    Function with Return Values

        In Rust, you need to define the Type of Return Value of a Function with an Arrow(->)*/
        fn five() -> i32{
            5
        }
        let x = five();
        println!("The value of x is: {}", x);

//      Functions can also have a Receiving Parameter and a Return like this:

        let x = plus_one(5);
        println!("The value of x is: {}", x);
        fn plus_one(x: i32) -> i32{
            x + 1 // if we would have here a semicolon the expression would change to a statement,
                  // and we would get an ERROR
        }

}

fn another_function(x : i32) { // Der Datatype muss im Parameter genannt werden
    println!("Value of X is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}