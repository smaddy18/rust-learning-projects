
enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

fn compute(op: Operation) -> Result<i32, &'static str> {
    match op {
        Operation::Add(left, right) => Ok(left + right),
        Operation::Subtract(left, right) => Ok(left - right),
        Operation::Multiply(left, right) => Ok(left * right),
        Operation::Divide(left, right) => {
            if right == 0 {
                Err("Division by zero")
            }else{
                Ok(left / right)
            }
        }
    }
}

fn main() {
    println!("{:?}", compute(Operation::Add(3, 4)));
    println!("{:?}", compute(Operation::Subtract(7, 4)));
    println!("{:?}", compute(Operation::Multiply(3, 2)));
    println!("{:?}", compute(Operation::Divide(10, 2)));
    println!("{:?}", compute(Operation::Divide(10, 0)));
}
