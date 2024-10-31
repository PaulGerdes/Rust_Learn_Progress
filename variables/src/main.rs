/*
fn main(){
    let x = 5;
    println!("The value of x is {}", x);
    x = 6 ;
    println!("The value of x is {}", x);
}

Output:
error[E0384]: cannot assign twice to immutable variable `x`

because x is not mutable so it only can defined once
 */

fn main() {
    let mut x = 5; // adding mut before x, so x is mutable (changable)
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    shadowing();
}

/*
Output:
The value of x is: 5
The value of x is: 6
 */

/*
The diference between variables and constants is that :
    1. Constants are defined with const and not let
    2. Constants don`t need mut because their always unchangable

Constants are used for fixed values like a max speed of a car in a game
Also is a constant valid for the entire time the program is running
 */

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// example for a constants
// you don`t use a variable because it`s a value wich never changes! (Like the count of secounds in 3 hours)


/*
 Shadowing is when a variable is created with the name of a before declared variable becuaus then
 the old variable gets in the shadow of the first one
 */
// Shadowing example :

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // The compiler always reads the last use / newest version
    // of the variable
    // this is why here is the output 6 instead of 5

    /*
    Output :
        The value of x in the inner scope is: 12
        The value of x is: 6
     */

    // when you try to use mut with shadowing it will not work because if a variable is mutable the
    // name or type connot get changed!
    // example :

    // working:
    let spaces = "   ";
    let spaces = spaces.len();

    /* not working:
    let mut spaces = "   ";
    spaces = spaces.len();
     */
}
