extern crate reddish;
use reddish::{random_int, random_float, random_choice, shuffle, sample, random_string, uuid, random_bool, random_bool_with_probability};

#[test]
fn test_random_int() {
    for _ in 0..100 {
        let result = random_int(1, 10);
        assert!(result >= 1 && result <= 10);
    }
}

#[test]
fn test_random_int_single_value() {
    for _ in 0..10 {
        let result = random_int(5, 5);
        assert_eq!(result, 5);
    }
}

#[test]
fn test_random_int_negative() {
    for _ in 0..100 {
        let result = random_int(-10, -1);
        assert!(result >= -10 && result <= -1);
    }
}

#[test]
#[should_panic(expected = "min cannot be greater than max")]
fn test_random_int_invalid_range() {
    random_int(10, 5);
}

#[test]
fn test_random_float() {
    for _ in 0..100 {
        let result = random_float(0.0, 1.0);
        assert!(result >= 0.0 && result < 1.0);
    }
}

#[test]
fn test_random_float_negative() {
    for _ in 0..100 {
        let result = random_float(-5.0, 5.0);
        assert!(result >= -5.0 && result < 5.0);
    }
}

#[test]
#[should_panic(expected = "min must be less than max")]
fn test_random_float_invalid_range() {
    random_float(5.0, 5.0);
}

#[test]
fn test_random_choice() {
    let vec = vec![1, 2, 3, 4, 5];

    for _ in 0..100 {
        let result = random_choice(&vec);
        assert!(result.is_some());
        assert!(vec.contains(result.unwrap()));
    }
}

#[test]
fn test_random_choice_empty() {
    let vec: Vec<i32> = vec![];
    let result = random_choice(&vec);
    assert!(result.is_none());
}

#[test]
fn test_random_choice_single_element() {
    let vec = vec![42];
    for _ in 0..10 {
        let result = random_choice(&vec);
        assert_eq!(result, Some(&42));
    }
}

#[test]
fn test_shuffle() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let original = vec.clone();

    shuffle(&mut vec);

    // Should have same length
    assert_eq!(vec.len(), original.len());

    // Should contain all original elements
    for item in &original {
        assert!(vec.contains(item));
    }

    // Should contain each element exactly once
    for item in &vec {
        assert_eq!(vec.iter().filter(|&x| x == item).count(), 1);
    }
}

#[test]
fn test_shuffle_empty() {
    let mut vec: Vec<i32> = vec![];
    shuffle(&mut vec);
    assert!(vec.is_empty());
}

#[test]
fn test_shuffle_single_element() {
    let mut vec = vec![42];
    shuffle(&mut vec);
    assert_eq!(vec, vec![42]);
}

#[test]
fn test_sample() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = sample(&vec, 5);
    assert_eq!(result.len(), 5);

    // All elements should be from original vector
    for item in &result {
        assert!(vec.contains(item));
    }

    // Should not contain duplicates
    for (_i, item) in result.iter().enumerate() {
        assert_eq!(result.iter().filter(|&x| x == item).count(), 1);
    }
}

#[test]
fn test_sample_more_than_available() {
    let vec = vec![1, 2, 3];
    let result = sample(&vec, 5);

    assert_eq!(result.len(), 3);
    for item in &vec {
        assert!(result.contains(item));
    }
}

#[test]
fn test_sample_empty() {
    let vec: Vec<i32> = vec![];
    let result = sample(&vec, 3);
    assert!(result.is_empty());
}

#[test]
fn test_sample_zero() {
    let vec = vec![1, 2, 3, 4, 5];
    let result = sample(&vec, 0);
    assert!(result.is_empty());
}

#[test]
fn test_random_string() {
    for length in [0, 1, 5, 10, 50] {
        let result = random_string(length);
        assert_eq!(result.len(), length);

        if length > 0 {
            assert!(result.chars().all(|c| c.is_alphanumeric()));
        }
    }
}

#[test]
fn test_random_string_uniqueness() {
    let mut strings = std::collections::HashSet::new();

    // Generate 100 random strings of length 10
    for _ in 0..100 {
        let s = random_string(10);
        strings.insert(s);
    }

    // Should have high uniqueness (allow for small chance of collision)
    assert!(strings.len() > 95);
}

#[test]
fn test_uuid() {
    let result = uuid();

    // Standard UUID format: 8-4-4-4-12 characters separated by hyphens
    assert_eq!(result.len(), 36);
    assert_eq!(result.matches('-').count(), 4);

    // Should be valid hex characters (except hyphens)
    let hex_part = result.replace('-', "");
    assert_eq!(hex_part.len(), 32);
    assert!(hex_part.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_uuid_uniqueness() {
    let mut uuids = std::collections::HashSet::new();

    // Generate 1000 UUIDs
    for _ in 0..1000 {
        let id = uuid();
        uuids.insert(id);
    }

    // All should be unique
    assert_eq!(uuids.len(), 1000);
}

#[test]
fn test_random_bool() {
    let mut true_count = 0;
    let mut false_count = 0;

    // Run many times to check distribution
    for _ in 0..1000 {
        if random_bool() {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }

    // Should be roughly 50/50 (allow for some variance)
    assert!(true_count > 400 && true_count < 600);
    assert!(false_count > 400 && false_count < 600);
    assert_eq!(true_count + false_count, 1000);
}

#[test]
fn test_random_bool_with_probability() {
    // Test with probability 0.0 (always false)
    for _ in 0..100 {
        assert_eq!(random_bool_with_probability(0.0), false);
    }

    // Test with probability 1.0 (always true)
    for _ in 0..100 {
        assert_eq!(random_bool_with_probability(1.0), true);
    }
}

#[test]
fn test_random_bool_with_probability_distribution() {
    let mut true_count = 0;

    // Test with 80% probability
    for _ in 0..1000 {
        if random_bool_with_probability(0.8) {
            true_count += 1;
        }
    }

    // Should be roughly 80% true (allow for variance)
    assert!(true_count > 750 && true_count < 850);
}

#[test]
#[should_panic(expected = "probability must be between 0.0 and 1.0")]
fn test_random_bool_with_probability_invalid_low() {
    random_bool_with_probability(-0.1);
}

#[test]
#[should_panic(expected = "probability must be between 0.0 and 1.0")]
fn test_random_bool_with_probability_invalid_high() {
    random_bool_with_probability(1.1);
}

// Integration tests
#[test]
fn test_random_workflow() {
    // Simulate a complete random workflow
    let options = vec!["option1", "option2", "option3", "option4"];

    // Pick a random option
    let chosen = random_choice(&options).unwrap();
    assert!(options.contains(chosen));

    // Generate random parameters
    let count = random_int(1, 10);
    let factor = random_float(0.1, 2.0);
    let enabled = random_bool();

    assert!(count >= 1 && count <= 10);
    assert!(factor >= 0.1 && factor < 2.0);
    assert!(enabled == true || enabled == false);

    // Create random identifier
    let id = uuid();
    assert_eq!(id.len(), 36);

    // Generate random data
    let data = random_string(20);
    assert_eq!(data.len(), 20);
}

#[test]
fn test_sampling_without_replacement() {
    let vec = vec![1, 2, 3, 4, 5];
    let sampled = sample(&vec, 3);

    // Check that no element appears twice
    for (i, &item) in sampled.iter().enumerate() {
        for (j, &other) in sampled.iter().enumerate() {
            if i != j {
                assert_ne!(item, other);
            }
        }
    }
}
