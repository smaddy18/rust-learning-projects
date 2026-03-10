
fn append_exclamation(input: &mut String){
    input.push('!');
}

fn main() {
    let mut str = String::from("Do it again");

    append_exclamation(&mut str);

    println!("{}", str);
}
