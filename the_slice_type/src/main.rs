// Normal way for finding spaces in a String
/*
fn first_word(s:&String) -> usize {
    let bytes = s.as_bytes(); // converts string in Array

    for (i, &item) in  bytes.iter().enumerate() { // iter outputs each element of a collection
        if item == b' '{    // &item because iter.enumerate gives a reverences of one part of the
            return i;       // tuple output from the enumerate
        }
    }
    s.len()// if no spaces are found in the for loop the length of the string will be returned
}
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
}
*/
// With String Slices:
/*
let s = String::from("hello");

let slice = &s[0..2]; These both are equally
let slice = &s[..2];

let s = String::from("hello");

let len = s.len();

let slice = &s[3..len]; These both are equally too
let slice = &s[3..];

If you drop both values you slice the entire String:
let slice = &s[..];
 */
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    let randomnumbers = [12, 25, 34, 455, 53, 66, 22 ,44];
    println!("{:?}",  intArraySlicer(&randomnumbers,2,5));

}

// Other Types of Slices

fn intArraySlicer(a: &[i32],s: usize, e: usize) -> &[i32]{
    if(e>s){
        return &a[s..e];
    }else {
        println!("Wrong Input E should be bigger than s");
        return &a[..];
    }

}

