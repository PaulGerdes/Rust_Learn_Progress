fn main() {
/*
    The most Common Way to change the Flow of the execution are if expressions and loops

    If Expressions:
        With if you can branch you code depending on conditions:*/
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

//      You can also use if Expressions in a Let Statement:
        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("The value of number is: {number}");

/*  Using Loops:
        loop {
            println!("again!");
        }*/
        // Returning Value with Loops
            let mut counter = 0;

            let result = loop {
                counter += 1;   // Everytime the loop starts it adding 1 to the counter Variable

                if counter == 10 {  // if counter is 10 it
                    break counter * 2;// breaks the loop and changes counter to counter x 2
                }
            };

            println!("The result is {result}");

//      Loop Labels to Disambiguate Between Multiple Loops
        // The first Loop is counting up from 0 to 2 and the secound is counting down from 10 to 9
            let mut count = 0;
            'counting_up: loop {
                println!("count = {count}");
                let mut remaining = 10;

                loop {
                    println!("remaining = {remaining}");
                    if remaining == 9 {     //if remaining is 9 the inner loop breaks
                        break;
                    }
                    if count == 2 {         // if count is 2 the outer loop breaks
                        break 'counting_up; // with a ' you can name a loop and den break and
                                            // continue it independently
                    }
                    remaining -= 1;
                }

                count += 1;
            }
            println!("End count = {count}");

//      Conditional Loops with while
            let mut number = 3;

            while number != 0 { //the while loop is only looping while number is not 0
                println!("{number}!");

                number -= 1;    // everytime the loop ends it subtract 1 from number
            }

            println!("LIFTOFF!!!");

//      Looping Through a Collection with for

            // the other way with while

            let a = [10, 20, 30, 40, 50];
            let mut index = 0;

            while index < 5 {   // it only loops while the index is smaller than 5
                println!("the value is: {}", a[index]);

                index += 1; // adding 1 to index at the end of loop
            }

//          the easy way with for:

                let a = [10, 20, 30, 40, 50];

                for element in a {
                    println!("the value is: {element}");
                }

            // another Example for a for loop use:
                for number in (1..4).rev() { // with .rev the range is reversed!
                    println!("{number}!");
                }
                println!("LIFTOFF!!!");
}
