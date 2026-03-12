
fn pig_latin(text: &str) -> String {
    let punctuations = ['.', ',', '!', '?', ';', ':'];
    let vowels = ['a', 'e', 'u', 'i', 'o', 'y'];

    let mut new_text = String::new();

    for word in text.split_whitespace() {
        let mut punctuation = "";

        let mut core_word = word;

        if word.ends_with(punctuations) {
            (core_word, punctuation) = word.split_at(word.len()-1);
        }
        
        if word.starts_with(vowels){
            if punctuation != "" {
                new_text.push_str(&format!(" {word}-hay{punctuation}"));
            }else {
                new_text.push_str(&format!(" {word}-hay"));
            }
        }else{
            let mut chars = core_word.chars();
            let first_char = chars.next().unwrap();
            let remained_word: String = chars.collect();
            if punctuation != "" {
                new_text.push_str(&format!(" {remained_word}-{first_char}ay{punctuation}"));
            }else{
                new_text.push_str(&format!(" {remained_word}-{first_char}ay"));
            }
        }
    }

    new_text.trim().to_string()
}

fn main() {
    let text = String::from("hello, world. How are things with you?");

    println!("{:?}", pig_latin(&text));
}
