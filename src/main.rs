use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the file path from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Read the contents of the file
    let contents = fs::read_to_string(file_path)?;

    // Parse the contents into a vector of numbers
    let numbers: Vec<i64> = contents
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    // Calculate statistics
    let average = calculate_average(&numbers);
    let median = calculate_median(&mut numbers.clone());
    let variance = calculate_variance(&numbers, average);
    let standard_deviation = (variance as f64).sqrt();

    // Print results
    println!("Average: {}", average);
    println!("Median: {}", median);
    println!("Variance: {}", variance);
    println!("Standard Deviation: {}", standard_deviation.round());

    Ok(())
}

fn calculate_average(numbers: &[i64]) -> i64 {
    let sum: i64 = numbers.iter().sum();
    sum / numbers.len() as i64
}

fn calculate_median(numbers: &mut [i64]) -> i64 {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        // For an even number of elements, average the two middle elements
        let middle_two = numbers[mid - 1] as f64 + numbers[mid] as f64;
        (middle_two / 2.0).round() as i64
    } else {
        // For an odd number of elements, return the middle element
        numbers[mid]
    }
}

fn calculate_variance(numbers: &[i64], mean: i64) -> i64 {
    let sum: i64 = numbers.iter().map(|&x| (x - mean).pow(2)).sum();
    sum / numbers.len() as i64
}