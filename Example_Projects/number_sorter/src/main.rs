use rand::Rng;

fn main() {
    let mut numbers: [i32; 10] = [0; 10];
    for i in 0..numbers.len(){
        numbers[i] = rand::thread_rng().gen_range(1..20);
        println!("{}", numbers[i]);
    }
    println!("\n");
    let mut swapped = true;
    while swapped {
        swapped = false; // If no elements are swapped its finished
        // Compare elements
        for i in 0..numbers.len() - 1 {
            if numbers[i] > numbers[i + 1] {
                // Swap elements
                numbers.swap(i, i + 1);
                swapped = true;
            }
        }
        // Print the array after each pass
        for num in &numbers {
            println!("{}", num);
        }
        println!("\n");
    }

}
