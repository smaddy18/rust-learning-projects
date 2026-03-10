
fn longest_word(sentence: &str) -> (&str, usize) {
    let mut longest = "";

    for word in sentence.split_whitespace() {
        if longest.len() < word.len() {
            longest = word;
        }
    }

    return (longest, longest.len());
}

fn main() {
    let input = "Rust makes systems programming safe and fast";

    let (longest_word, length) = longest_word(input);

    println!("longest word : {} with length : {}", longest_word, length);
}
