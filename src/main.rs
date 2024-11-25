use core::num;
use rand::{distributions::Uniform, Fill, Rng};
use std::{io::stdin, usize};

fn main() -> Result<(), std::io::Error> {
    'main: loop {
        let mut input = String::new();

        println!("Please enter your roll in the form of [num]d[size][+/-][modifier]. Examples: 2d10, 3d6+2, 1d20-4");
        println!("You may enter [q] to quit");

        stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "q" {
            break 'main;
        }
        let mut d_index: usize = 0;
        let mut operation = ' ';
        let mut operation_index = 0;

        for char in input.trim().char_indices() {
            if char.1 == 'd' {
                if d_index == 0 {
                    d_index = char.0;
                } else {
                    println!("Invalid format. Character multiples or missing [d] character.");
                }
            }
            if char.1 == '+' {
                if operation_index == 0 {
                    operation = '+';
                    operation_index = char.0;
                } else {
                    println!("Invalid format. Multiple operations not allowed.");
                }
            }

            if char.1 == '-' {
                if operation_index == 0 {
                    operation = '-';
                    operation_index = char.0;
                } else {
                    println!("Invalid format. Multiple operations not allowed.");
                }
            }
        }
        let (number_result, suffix_result) = input.trim().split_at(d_index);
        let mut size_result: &str = "";
        let mut modifier_result: &str = "";

        if operation_index != 0 && (operation == '+' || operation == '-') {
            (size_result, modifier_result) = suffix_result.split_once(operation).unwrap();
        } else {
            size_result = suffix_result;
        }
        let number = number_result.parse::<usize>().unwrap_or(0);
        let size = 1 + size_result.replace("d", "").parse::<usize>().unwrap_or(0);
        let modifier = modifier_result.parse::<usize>().unwrap_or(0);

        if number == 0 {
            println!("Invalid format. Unable to determine number of dice. Enter a positive whole number before the [d].");
        } else if size == 0 {
            println!("Invalid format. Unable to determine size of dice. Enter a positive whole number after the [d].");
        } else {
            let range = Uniform::new(1, size);

            let results: Vec<usize> = rand::thread_rng()
                .sample_iter(&range)
                .take(number)
                .collect();

            let sum = results.clone().into_iter().sum::<usize>();

            if operation == '-' {
                if modifier > sum {
                    println!(
                        "Total: {} (Each die: {:?}, {}{} modifier)",
                        0, results, operation, modifier
                    );
                } else {
                    println!(
                        "Total: {} (Each die: {:?}, {}{} modifier)",
                        sum - modifier,
                        results,
                        operation,
                        modifier
                    );
                }
            } else if operation == '+' {
                println!(
                    "Total: {} (Each die: {:?}, {}{} modifier)",
                    sum + modifier,
                    results,
                    operation,
                    modifier
                );
            }
        }
    }
    Ok(())
}

fn report_results(sum: usize, results: Vec<usize>, operation: char, modifier: usize) {}
