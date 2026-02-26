fn main() {
    
    
    

    // Convert Degree Celsius -> Kelvin
    // K = C + 273.15

    // Convert Degree Kelvin -> Celsius
    // C = K - 273.15 = Absolute zero when K = 0° -> C = -273.15

    // Convert Degree Kelvin -> Fahrenhein
    // F = (K × (9/5)) - 459.67

    // Convert Degree Fahrenheit -> Kelvin
    // K = 5/9 * (F + 459.67)
}

//convert Degree Celsius -> Fahrenheit 
// F = (C * 9/5) + 32
fn celsius_to_fahrenheit(&celsius: f64) -> f64 {
    (celsius * (9/5)) + 32;
}

// Convert Degree Fahrenheit -> Celsius
// C = 5/9 * (F-32)
fn fahrenheit_to_celsius(&celsius: f64)
