
use std::io;

fn main() {
    'loopw: loop {
        let mut user_input_temp = String::new();
        println!("Bitte Temperatur eingeben");
        io::stdin()

            .read_line(&mut user_input_temp)

            .expect("Failed to read line");
        let user_input_temp: f64 = match user_input_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        let mut user_input = String::new();
        println!("Zwischen 3 Moeglichkeiten waehlen:\n1. Grad in Fahreneinheit\n2. Fahreneinheit in Grad");
        io::stdin()

            .read_line(&mut user_input)

            .expect("Failed to read line");
        let user_input: f64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    if user_input > 3.0  {
            println!("Bitte waehle eine Moeglichkeit zwischen 1 und 3");
            continue 'loopw;
        }

        if user_input == 1.0{
            println!("Grad zu fahreneinheit wird gerechnet!");
            let result: f64 = {
                let usertemp: f64 = user_input_temp as f64;
                usertemp * 1.8 + 32.0
            };
            println!("Das Ergebnis in fahreneinheit ist: {}", result);
        }
        if user_input == 2.0{
            println!("fahreneinheit zu Grad wird gerechnet!");
            let result: f64 = {
                let _usertemp: f64 = user_input_temp as f64;
                let helpvalue: f64 = 0.0;
                (helpvalue - 32.0 )/1.8
            };
            println!("Das Ergebnis in Grad ist: {}", result);
        }
    }


}
