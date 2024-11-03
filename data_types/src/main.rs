fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // without u32 you will get an error because rust needs more information about the type of the
    // variable

    /*  Scalar Types:
            represent single values
            there are 4 primary scalar types:
                integers, floating-point numbers, Booleans, and characters

- - - - - - - -  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    Integers:
        There are two types of integers:
            1.  signed integers
            2.  unsigned integers
        the differences between them are their possible numbers in the binary system.

        Signed Values can store from -(2ⁿ⁻¹) to 2ⁿ⁻¹-1
        Unsigned Values can store from 0 to 2ⁿ

        As an Example a 4-bit Integer:

            unsigned:      signed:
            bits  value |  bits  value
            0000    0   |  0000    0
            0001    1   |  0001    1
            0010    2   |  0010    2
            0011    3   |  0011    3
            0100    4   |  0100    4
            0101    5   |  0101    5
            0110    6   |  0110    6
            0111    7   |  0111    7
            1000    8   |  1000   -8
            1001    9   |  1001   -7
            1010   10   |  1010   -6
            1011   11   |  1011   -5
            1100   12   |  1100   -4
            1101   13   |  1101   -3
            1110   14   |  1110   -2
            1111   15   |  1111   -1

        There are also different Integer Types in Rust with different length/sizes:

            Length      Signed      Unsigned
            8-bit       i8          u8
            16-bit      i16         u16
            32-bit      i32         u32
            64-bit      i64         u64
            128-bit     i128        u128
            arch        isize       usize   The i/usize Integer depends on the system of the
                                            Computer so 64-bit or 32-bit architecture!


        There are also different Literal Integer Types in Rust with different suffix's:
            Number literals    Example
            Decimal            98_222
            Hex                0xff
            Octal              0o77
            Binary             0b1111_0000
            Byte(u8 only)      b'A'

    Floating-Point Types:
     */
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
        // So there are only two types of floating integers in Rust

//  Numeric Operations:
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1

        // remainder
        let remainder = 43 % 5;

//  Boolean Type:
        let t = true;

        let f: bool = false; // with explicit type annotation

//  Character Type:
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';

/*  Compound Types:
        Compound Types can store multiple Values into one type. There are 2 Types:

            1. Tuple
                Tuple's can not grow or shrink in size(onetime declared size)
                Tuple's Values don't need to be the same type of Variable
                Example:
     */             let tup: (i32, f64, u8) = (500, 6.4, 1);

//              Also you can destruct a Tuple like this:
                    let tup = (500, 6.4, 1);
                    let (x, y, z) = tup;
                    println!("The value of y is: {}", y);

//              You can also Access a Tuple directly by using a period(.)
                    let x: (i32, f64, u8) = (500, 6.4, 1);
                    let five_hundred = x.0;
                    let six_point_four = x.1;
                    let one = x.2;

//              A Tuple without any Values is named Unit
/*
            2. Arrays:
                Every Element of an Array need's to be the same Type
                Arrays have in Rust a fixed length too

                Examples of Arrays:
 */                 let a = [1, 2, 3, 4, 5];

                    let months = ["January", "February", "March", "April", "May", "June",
                        "July", "August", "September", "October", "November", "December"];

                    let a: [i32; 5] = [1, 2, 3, 4, 5];

//                  But there is a Special Syntax for Arrays!
                    let a = [3; 5]; //This is the same as:
                    let a = [3, 3, 3, 3, 3];


}
