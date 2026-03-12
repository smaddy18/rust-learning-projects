use std::collections::HashMap;

fn add_student(name: &str, grade: f64, grades: &mut HashMap<String, f64>) {
    grades.entry(name.to_string()).or_insert(grade);
}

fn update_grade(name: &str, new_grade: f64, grades: &mut HashMap<String, f64>) {
    if let Some(grade) = grades.get_mut(name) {
        *grade = new_grade;
    }
}

fn get_grade(name: &str, grades: &HashMap<String, f64>) -> f64 {
    grades.get(name).copied().unwrap_or(0.0)
}

fn print_all(grades: &HashMap<String, f64>) {
    for (name, grade) in grades {
        println!("{}: {}", name, grade);
    }
}

fn main() {
    let mut grades = HashMap::new();

    add_student("John", 12.50, &mut grades);
    add_student("Francky", 18.00, &mut grades);
    add_student("Mary", 16.50, &mut grades);

    print_all(&grades);

    update_grade("John", 14.00, &mut grades);

    println!("John's new grade is: {}", get_grade("John", &grades));
}
