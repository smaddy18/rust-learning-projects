
fn first_sentence(text: &str) -> &str {
    if let Some(index) = text.find('.') {
        &text[..index]
    }else{
        text
    }
}

fn main() {
    let text = "Rust is great. It is memory safe.";

    let first_sentence = first_sentence(text);

    println!("{}", first_sentence);
}
