fn main() {

}
// Variable Scope
fn s_test(){ // s is not declared now so not valid
    let s = "hello";// from here s is declared and valid
    // here you can do something with s

}// this is out of the scope and now s is no more valid

// The String Type

// let s = String::from("hello");
// with that you declare a string in a string with the from function! Strings like this are mutable!

// Example:

fn string_in_string_mutable(){
    let mut s = String::from("hello"); // the output from this string would only be hello
    s.push_str(", world");// in a string declared in a string you can push data like this
    println!("{}", s); // the output would now be hello, world
}

// Memory and Allocation
     // in most other low level languages you need to free the memory wich is full from the mutable
    // string your self, but in rust it does it for you! Just by creating a } the drop function is
    // called wich is in other words a free function

// Variables and Data Interacting with Move

// Lets look at an Example:
fn data_move() {
    let x = 5;
    let y = x;// in this code the Variable x is getting copied to the variable y and both are
                   // the same now!

    let s1 = String::from("hello");
    let s2 = s1;// this string wich is the same s1 looks the same and you would think it
                       // would also work the same as at the integers but this is wrong!
    /*
    in reality you have the stack and heap and a string is stored in both!

    in the stack are 4 information about the string: 1.name
                                                     2.ptr to the heap memory
                                                     3. length
                                                     4. capacity

    in the heap are stored the same amount of values as the length of the string!
    Because you got the index and value. in the index is the Memory "named" and in the value are the
    individual characters of the String. So like this : index value
                                                          0     h
                                                          1     e
                                                          2     l
                                                          3     l
                                                          4     o

    The pointer points to the index of the first char of the string(0 = h), the length defines how
    long the computer needs to read the index's to get the whole string

    now if you "copie" the String like in the beginning there are now not 2 stack Saves and 2 Heap
    saves! Rust only needs 1 Heap save and 2 Strings in the Stack. From both of the Strings in the
    Stack are the pointer pointing to the same start index in the heap.


   But if you want to deeply clone the String not only in the Stack(also in heap) you can use this
   Syntax:*/
    let s1 = String::from("hello");
    let s2 = s1.clone(); // with clone, you create new stack and heap data!

    println!("s1 = {s1}, s2 = {s2}");


}
