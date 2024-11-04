
use std::io;

fn main() {
    loop {
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
            println!("Bitte wähle eine Möglichkeit zwischen 1 und 3");
            continue;
        }

        if user_input == 1.0{
            println!("Grad zu Fahreneinheit wird gerechnet!");
            let result = {

                user_input_temp * 1.8 + 32.0
            };
            println!("Das Ergebnis in Fahreneinheit ist: {}", result);
        }

        if user_input == 2.0{
            println!("Fahreneinheit zu Grad wird gerechnet!");
            let result = {
                (user_input_temp - 32.0 )/1.8
            };
            println!("Das Ergebnis in Grad ist: {}", result);
        }
    }


}
