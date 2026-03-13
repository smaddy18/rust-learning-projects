
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {

    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];

    for element in list {
        if largest < element { // comment c'est possible que je puisse comparer deux ref au lieu des valeur ?
            largest = element;
        }
    }
    
    Some(largest)
}

fn main() {
    let letters = vec!["Franck", "John", "Mary"];
    let result = largest(&letters);

    if let Some(element) = result {
        println!("{element}");
    }
}
