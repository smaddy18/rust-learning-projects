use std::io;

fn ask_user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn generate_nth_number(n: u64) -> u64 {
    if n == 0{
        println!("The {}th number doesn't exist. Enter 1 for exemple for the 1st number !", n);
        return 0;
    }else if n == 1 {
        return 0;
    }else if n == 2 || n == 3 {
        return 1;
    }

    let mut Fn_1 = 1;
    let mut Fn_2 = 1;
    let mut Fn_i = 0;
    let mut index = 4;
    while index <= n {
        Fn_i = Fn_1 + Fn_2;

        Fn_2 = Fn_1;
        Fn_1 = Fn_i;

        index += 1;
    }
    return Fn_i;
}

fn main() {
    println!("---------- GENERATE THE nth FIBONACCI NUMBER ----------");

    let mut continue_generating = true;
    while continue_generating {
        println!("---------- Enter the nth Fibonacci number you want OR enter QUIT to end the programm ----------");

        let user_input = ask_user_input();
        let user_input = user_input.trim();

        if user_input.to_uppercase() == "QUIT" {
            continue_generating = false;
            println!("Bye... !!!")
        }else{
            let user_input: u64 = user_input.parse().expect("Failed convert your input. Please try again");
            let nth_number = generate_nth_number(user_input);
            
            match user_input {
                1 => println!("The {}st Fibonacci number is : {}", user_input, nth_number),
                2 => println!("The {}nd Fibonacci number is : {}", user_input, nth_number),
                3 => println!("The {}rd Fibonacci number is : {}", user_input, nth_number),
                other => println!("The {}th Fibonacci number is : {}", other, nth_number),
            }
        }
    }
}
