extern crate reddish;
use reddish::{chunk, flatten, group_by, unique, partition, zip, count_by};

fn main() {
    println!("=== Collection Methods Examples ===\n");

    // Demonstrate chunk() function
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let chunks = chunk(&numbers, 3);
    println!("chunk([1,2,3,4,5,6,7,8,9], 3): {:?}", chunks);
    // chunk([1,2,3,4,5,6,7,8,9], 3): [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

    // Demonstrate flatten() function
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened = flatten(&nested);
    println!("flatten([[1,2], [3,4], [5,6]]): {:?}", flattened);
    // flatten([[1,2], [3,4], [5,6]]): [1, 2, 3, 4, 5, 6]

    // Demonstrate unique() function
    let with_duplicates = vec![1, 2, 2, 3, 1, 4, 3, 5];
    let unique_values = unique(&with_duplicates);
    println!("unique([1,2,2,3,1,4,3,5]): {:?}", unique_values);
    // unique([1,2,2,3,1,4,3,5]): [1, 2, 3, 4, 5]

    // Demonstrate partition() function
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let (evens, odds) = partition(&numbers, |&n| n % 2 == 0);
    println!("partition([1,2,3,4,5,6,7,8], is_even):");
    println!("  evens: {:?}", evens);
    println!("  odds: {:?}", odds);
    // evens: [2, 4, 6, 8]
    // odds: [1, 3, 5, 7]

    // Demonstrate zip() function
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let zipped = zip(&names, &ages);
    println!("zip([\"Alice\", \"Bob\", \"Charlie\"], [25, 30, 35]): {:?}", zipped);
    // zip(["Alice", "Bob", "Charlie"], [25, 30, 35]): [("Alice", 25), ("Bob", 30), ("Charlie", 35)]

    // Demonstrate group_by() function
    let words = vec!["apple", "banana", "apricot", "blueberry", "avocado"];
    let grouped = group_by(&words, |s| s.chars().next().unwrap());
    println!("group_by([\"apple\", \"banana\", \"apricot\", \"blueberry\", \"avocado\"], first_char):");
    for (key, group) in &grouped {
        println!("  '{}': {:?}", key, group);
    }
    // 'a': ["apple", "apricot", "avocado"]
    // 'b': ["banana", "blueberry"]

    // Demonstrate count_by() function
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let counts = count_by(&numbers, |&n| n % 3);
    println!("count_by([1,2,3,4,5,6,7,8,9,10], n % 3):");
    for (remainder, count) in &counts {
        println!("  remainder {}: {} numbers", remainder, count);
    }
    // remainder 0: 3 numbers (3, 6, 9)
    // remainder 1: 4 numbers (1, 4, 7, 10)
    // remainder 2: 3 numbers (2, 5, 8)
}
