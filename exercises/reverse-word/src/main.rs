// extern crate unicode_segmentation;
// use unicode_segmentation::UnicodeSegmentation;
// use time::{PrimitiveDateTime as DateTime, Date, ext::NumericalDuration};
// use time_macros::datetime;

fn main() {
    let word = "Goodbye";

    // // let mut output = String::new();
    
    let output = reverse(word);
    println!("{:?}", output);



    // let dt = datetime!(2011-4-25 00:00:00);
    // println!("{:?}", after(dt));
}

fn reverse(input: &str) -> String {
    // let mut output = String::new();

    // for char in input.chars().rev() {
    //     output = format!("{}{}", output, char);
    // }

    // return output;

    return input.chars().rev().collect();
}

// fn after(start: DateTime) -> DateTime {
//     let moment_after = start.checked_add(1000000000.seconds());
//     match moment_after {
//         Some(moment) => moment,
//         other => other.expect("Enter a valid datetime")
//     }
// }