
fn main() {
    enum IpAddrkind {
        V4(String),
        V6(String),
    }

    // Enum Values
    let four = IpAddrkind::V4;
    //let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrkind) {} // this function would take both IpAddrKind's
    /*
    We would call the function like this for both variants
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
     */

    struct IpAddr {
        kind: IpAddrkind,
        address: String,
    }
/*
    Normal Version with Struct:

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
*/
    //Easy version without Struct:

    //let home = IpAddr::V4(String::from("127.0.0.1"));

    //let loopback = IpAddr::V6(String::from("::1"));

    enum IpAddrenum {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrenum::V4(127, 0, 0, 1);

    let loopback = IpAddrenum::V6(String::from("::1"));

    // another Example

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // This Enum got 4 Different Variants with different Types
    /*The same as Enum Message in Struct form:
        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct
     */
    // You can also define Methods on Enum's
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Another Type of Enum is the Option Type
    // This Type is for the Scenario when a value could be anything but also nothing

    enum Option<T> {
        None,
        Some(T),
    }



    // The match control flow construct
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(value: i32) -> i32 {
        match coin{
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 0,
        }
    }

    // Paterns that bind to Values
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents2(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }



    // Matching with Option<T>

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);




    // Catch-all Patterns and the _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        _ => reroll(),
        _ => (),
        // 3 different types the underline means that we dont use any other values than the used ones
        // in the top
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

}


