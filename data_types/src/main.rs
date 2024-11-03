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
     */
}
