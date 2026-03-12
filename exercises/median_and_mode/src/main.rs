use std::collections::HashMap;
fn statistics(numbers: &Vec<i32>) -> (i32, i32) {
    let median = get_median(numbers);
    let mode = get_mode(numbers);

    (median, mode)
}

fn get_median(numbers: &Vec<i32>) -> i32 {
    let mut sorted = numbers.clone();
    sorted.sort();
    sorted[sorted.len() / 2]
}

fn get_mode(numbers: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for number in numbers {
        let count = counts.entry(number).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut mode_count = 0;
    
    for (number, count) in &counts {
        if mode_count < *count {
            mode = **number;
            mode_count = *count;
        }
    }

    mode
}

fn main() {
   let numbers = vec![1,2,2,3,4];

   let (median, mode) = statistics(&numbers);

   println!("median: {}\nmode: {}", median, mode);
}
