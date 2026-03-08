use std::collections::HashMap;

fn main() {
    let mut list = vec![2, 5, 1, 3, 3, 9, 5, 3];

    // Median
    list.sort();
    let median = list[list.len()/2];
    println!("list : {:?} and median is : {:?}", list, median);

    // Mode
    let mut counts = HashMap::new();

    for number in &list {
        let count = counts.entry(number).or_insert(0);
        *count += 1;
    }

    // Find the maximum count

    let mut mode = list[0];
    let mut max_count = 0;

    for (number, count) in &counts {
        if *count > max_count {
            max_count = *count;
            mode = **number;
        }
    }

    // Display
    for (number, count) in counts {
        if *number == mode {
            println!("{} -> {} -> mode", number, count);
        }else{
            println!("{} -> {}", number, count);
        }
    }
}
