use std::env;
use std::fs;
use std::str::SplitAsciiWhitespace;

fn main() {
    let solution : u32 = 0;
    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut list_one: Vec<u32> = vec![];
    let mut list_two: Vec<u32> = vec![];

    for line in  input.lines(){
        let mut split: SplitAsciiWhitespace<'_> = line.split_ascii_whitespace();
        let (Some(first_word), Some(second_word)) = (split.next(), split.next()) else {
            continue;
        };
        Some(list_one.push(first_word.parse().unwrap()));
        Some(list_two.push(second_word.parse().unwrap()));
    }

    for value in list_one.iter(){

        let minValue_one = *list_one.iter().min().unwrap();
        for value in list_two.iter(){

            let minValue_two = *list_two.iter().min().unwrap();
            (if minValue_one > minValue_two {
                solution + minValue_one - minValue_two;
            }else{
                solution + minValue_two - minValue_one;
            });


        }

    }
    println!("{}", solution);
}

