use std::collections::HashMap;

/*
    Homework from chapter 8

    Given a list of integers, use a vector and return the median (when sorted, the value in the
    middle position) and mode (the value that occurs most often; a hash map will be helpful here)
    of the list.
 */
fn main() {
    // let integers = vec![1, 23, 19, 22, 11, 30, 49, 77, 30, 93, 71, 60, 50, 88, 87, 90,];
    // let integers = vec![1, 23, 19, 22, 11, 30, 49, 77, 30, 93, 71, 60, 50, 88, 87, 90, 99];
    // let integers = vec![0];
    let integers = vec![0, 0, 1, 2, 3];

    let (average, median, mode) = (
        calculate_average(&integers),
        find_median(&integers),
        find_mode(&integers),
    );

    println!("Average: {average}");
    if let Some(median) = median {
        println!("Median: {median}");
    } else {
        println!("Median: None");
    }
    println!("Mode: {mode}");
}

fn calculate_average(integers: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for number in integers {
        sum += number;
    }
    sum as f32 / integers.len() as f32
}

fn find_median(integers: &Vec<i32>) -> Option<i32>{
    let mut sorted_integers = integers.clone();
    sorted_integers.sort();

    match integers.len() % 2 {
        0 => {
            // Even number of integers so just get the middle element
            let middle_index = integers.len() / 2;

            let middle_number = sorted_integers.get(middle_index);

            match middle_number {
                Some(number) => Some(*number),
                None => None
            }
        }
        _ => {
            // Early Exit: If length of integers is 1 the median can only be 1 number
            if integers.len() == 1 {
                let middle_number = sorted_integers.get(0);
                return match middle_number {
                    Some(number) => Some(*number),
                    None => None
                }
            }

            // Odd number of integers so get the elements on both sides of the middle and return
            // the difference
            let middle_index = integers.len() / 2;

            let first_number = sorted_integers.get(middle_index);
            let second_number = sorted_integers.get(middle_index + 1);

            match (first_number, second_number) {
                (Some(first_number), Some(second_number)) => {
                    Some((*first_number + *second_number) / 2)
                }
                _ => None
            }

        }
    }
}

fn find_mode(integers: &Vec<i32>) -> i32 {
    let mut number_frequency = HashMap::new();

    for number in integers {
        // return the count or insert 0 into the hashmap and return it
        let count = number_frequency.entry(number).or_insert(0);
        *count += 1;
    }

    let mut frequency_count = 0;
    let mut highest_frequency = 0;

    for (number, frequency) in &number_frequency {
        if *frequency > frequency_count {
            frequency_count = *frequency;
            highest_frequency = **number;
        }
    }

    highest_frequency
}