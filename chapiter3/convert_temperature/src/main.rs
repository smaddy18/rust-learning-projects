use std::io;

fn main() {
    println!("---------- TEMPERATURE CONVERTER ----------");

    let menu_options = ['A', 'B', 'C', 'D', 'E', 'F'];
    let mut quit_converter = false;

    while !quit_converter {

        // let mut option_choosen = String::new();


        //---------- Display the menu ----------
        println!("---------- Menu ----------");
        println!("A : Celsius -> Fahrenheit");
        println!("B : Fahrenheit -> Celsius");
        println!("C : Celsius -> Kelvin");
        println!("D : Kelvin -> Celsius");
        println!("E : Kelvin -> Fahrenheit");
        println!("F : Fahrenheit -> Kelvin");

        println!("---------- Choose a converter ----------");
        
        // io::stdin()
        // .read_line(&mut option_choosen)
        // .expect("Failed to read line");
        let choice = ask_user_input();
        let choice: char = choice[..1]
            .trim()
            .to_uppercase()
            .parse()
            .expect("Make a valid choice");

        if !menu_options.contains(&choice) {
            continue;
        }

        let mut convertion = true;
        while convertion {

            println!("---------- Enter a temperature to convert OR Q to quit OR M to return to the menu----------");

            let mut user_temp = String::new();

            if choice == 'A' {
                println!("You chose A : Celsius -> Fahrenheit");
                user_temp = ask_user_input().trim().to_string();
                let (converted_temp, quit, menu) = converting_temp(celsius_to_fahrenheit, &user_temp);
                if quit {
                    convertion = false;
                    quit_converter = true;
                }else if menu {
                    convertion = false;
                }else {
                    println!("{}°C = {}°F", user_temp, converted_temp);
                }
            }else if choice == 'B' {
                println!("You chose B : Fahrenheit -> Celsius");
                user_temp = ask_user_input().trim().to_string();
                let (converted_temp, quit, menu) = converting_temp(fahrenheit_to_celsius, &user_temp);
                if quit {
                    convertion = false;
                    quit_converter = true;
                }else if menu {
                    convertion = false;
                }else {
                    println!("{}°F = {}°C", user_temp, converted_temp);
                }
            }else if choice == 'C' {
                println!("You chose C : Celsius -> Kelvin");
                user_temp = ask_user_input().trim().to_string();
                let (converted_temp, quit, menu) = converting_temp(celsius_to_kelvin, &user_temp);
                if quit {
                    convertion = false;
                    quit_converter = true;
                }else if menu {
                    convertion = false;
                }else {
                    println!("{}°C = {}°K", user_temp, converted_temp);
                }
            }else if choice == 'D' {
                println!("You chose D : Kelvin -> Celsius");
                user_temp = ask_user_input().trim().to_string();
                let (converted_temp, quit, menu) = converting_temp(kelvin_to_celsius, &user_temp);
                if quit {
                    convertion = false;
                    quit_converter = true;
                }else if menu {
                    convertion = false;
                }else {
                    println!("{}°K = {}°C", user_temp, converted_temp);
                }
            }else if choice == 'E' {
                println!("You chose E : Kelvin -> Fahrenheit");
                user_temp = ask_user_input().trim().to_string();
                let (converted_temp, quit, menu) = converting_temp(kelvin_to_fahrenheit, &user_temp);
                if quit {
                    convertion = false;
                    quit_converter = true;
                }else if menu {
                    convertion = false;
                }else {
                    println!("{}°K = {}°F", user_temp, converted_temp);
                }
            }else if choice == 'F' {
                println!("You chose F : Fahrenheit -> Kelvin");
                user_temp = ask_user_input().trim().to_string();
                let (converted_temp, quit, menu) = converting_temp(fahrenheit_to_kelvin, &user_temp);
                if quit {
                    convertion = false;
                    quit_converter = true;
                }else if menu {
                    convertion = false;
                }else {
                    println!("{}°F = {}°K", user_temp, converted_temp);
                }
            }
        }
    }
}



fn converting_temp(f: fn(&f64) -> f64, user_temp: &str) -> (f64, bool, bool){
    let user_decision: char = user_temp[..1]
        .trim()
        .to_uppercase()
        .parse()
        .expect("Make a valid choice");
    
    if user_decision == 'Q' {
        return (0.0, true, false);
    }else if user_decision == 'M' {
        return (0.0, false, true);
    }else {
        let user_temp: f64 = user_temp.trim().parse().expect("Failed to convert your input");
        let converted_temp = f(&user_temp);
        return (converted_temp, false, false);
        // println!("{}°C = {}°F", user_temp, converted_temp);
    }
}

//convert Degree Celsius -> Fahrenheit 
// F = (C * 9/5) + 32
fn celsius_to_fahrenheit(celsius: &f64) -> f64 {
    (celsius * (9.0/5.0)) + 32f64
}

// Convert Degree Fahrenheit -> Celsius
// C = 5/9 * (F-32)
fn fahrenheit_to_celsius(fahrenheit: &f64) -> f64 {
    (5.0/9.0) * (fahrenheit - 32f64)
}

// Convert Degree Celsius -> Kelvin
// K = C + 273.15
fn celsius_to_kelvin(celsius: &f64) -> f64 {
    celsius + 273.15
}

// Convert Degree Kelvin -> Celsius
// C = K - 273.15 = Absolute zero when K = 0° -> C = -273.15
fn kelvin_to_celsius(kelvin: &f64) -> f64 {
    kelvin - 273.15
}

// Convert Degree Kelvin -> Fahrenhein
// F = (K × (9/5)) - 459.67
fn kelvin_to_fahrenheit(kelvin: &f64) -> f64 {
    (kelvin * (9.0/5.0)) - 459.67
}

// Convert Degree Fahrenheit -> Kelvin
// K = 5/9 * (F + 459.67)
fn fahrenheit_to_kelvin(fahrenheit: &f64) -> f64 {
    (5.0/9.0) * (fahrenheit + 459.67)
}

fn ask_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    return input;
}
