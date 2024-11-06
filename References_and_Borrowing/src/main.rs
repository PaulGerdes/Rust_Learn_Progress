fn main() {
    let s1 = String::from("hello");
    let _len = calculate_length(&s1); // here we borrow the value of s1 with &
    mutable_references();
    multible_mutable_reference();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

fn mutable_references(){
    let mut s = String::from("hello");// first we change s to mutable
    change(&mut s);
}

fn change(some_string: &mut String) { // change the function signature to accept a mutable reference
    some_string.push_str(", world");
}

/*
    The next example will not work because you can only create for one variable one mutable reference
    at a time. But at the moment when you used the mutable reference in a Println function you can
    create another one!

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);


    This happens because don't let you make data races these happen at three behaviors:
    1.Two or more pointers access the same data at the same time.
    2.At least one of the pointers is being used to write to the data.
    3.There’s no mechanism being used to synchronize access to the data.

    So if it would be like this it would work:
 */
fn multible_mutable_reference(){
    let mut s = String::from("hello");
    let r1 = &s;// no problem
    let r2 = &s;// no problem
    //let r3 = &mut s;  BIG PROBLEM
    println!("{} and {}", r1, r2);
}

fn dangling_references(){

}