extern crate reddish;
use reddish::{random_int, random_float, random_choice, shuffle, sample, random_string, uuid, random_bool, random_bool_with_probability};

fn main() {
    println!("=== Random Methods Examples ===\n");

    // Demonstrate random_int() function
    println!("Random integers:");
    for _ in 0..5 {
        let num = random_int(1, 100);
        println!("  Random int (1-100): {}", num);
    }
    println!();

    // Demonstrate random_float() function
    println!("Random floats:");
    for _ in 0..5 {
        let num = random_float(0.0, 1.0);
        println!("  Random float (0.0-1.0): {:.4}", num);
    }
    println!();

    // Demonstrate random_choice() function
    let colors = vec!["red", "green", "blue", "yellow", "purple"];
    println!("Random choices from {:?}:", colors);
    for _ in 0..5 {
        if let Some(color) = random_choice(&colors) {
            println!("  Chosen color: {}", color);
        }
    }
    println!();

    // Demonstrate shuffle() function
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original numbers: {:?}", numbers);
    shuffle(&mut numbers);
    println!("Shuffled numbers: {:?}", numbers);
    println!();

    // Demonstrate sample() function
    let fruits = vec!["apple", "banana", "cherry", "date", "elderberry", "fig", "grape"];
    println!("All fruits: {:?}", fruits);
    let sampled = sample(&fruits, 3);
    println!("Random sample of 3: {:?}", sampled);
    println!();

    // Demonstrate random_string() function
    println!("Random strings:");
    for length in [5, 10, 15] {
        let random_str = random_string(length);
        println!("  Length {}: {}", length, random_str);
    }
    println!();

    // Demonstrate uuid() function
    println!("Random UUIDs:");
    for _ in 0..3 {
        let id = uuid();
        println!("  UUID: {}", id);
    }
    println!();

    // Demonstrate random_bool() function
    println!("Random booleans:");
    for _ in 0..10 {
        let boolean = random_bool();
        print!("{} ", boolean);
    }
    println!("\n");

    // Demonstrate random_bool_with_probability() function
    println!("Random booleans with different probabilities:");

    print!("90% true:  ");
    for _ in 0..10 {
        let boolean = random_bool_with_probability(0.9);
        print!("{} ", boolean);
    }
    println!();

    print!("10% true:  ");
    for _ in 0..10 {
        let boolean = random_bool_with_probability(0.1);
        print!("{} ", boolean);
    }
    println!();

    // Practical examples
    println!("\n=== Practical Use Cases ===");

    // Random password generation
    let password = random_string(12);
    println!("Generated password: {}", password);

    // Random user ID
    let user_id = uuid();
    println!("User ID: {}", user_id);

    // Random game dice
    let dice1 = random_int(1, 6);
    let dice2 = random_int(1, 6);
    println!("Dice roll: {} + {} = {}", dice1, dice2, dice1 + dice2);

    // Random team selection
    let players = vec!["Alice", "Bob", "Charlie", "Diana", "Eve", "Frank"];
    let team = sample(&players, 3);
    println!("Random team: {:?}", team);

    // Random quiz question
    let questions = vec![
        "What is 2+2?",
        "What is the capital of France?",
        "Who wrote Romeo and Juliet?",
        "What is the largest planet?",
    ];
    if let Some(question) = random_choice(&questions) {
        println!("Random quiz question: {}", question);
    }

    // Random A/B test assignment
    let is_variant_a = random_bool();
    println!("A/B Test assignment: {}", if is_variant_a { "Variant A" } else { "Variant B" });
}
