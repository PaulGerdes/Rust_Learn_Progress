
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

}
