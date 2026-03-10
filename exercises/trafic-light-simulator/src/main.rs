
enum TraficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TraficLight) -> &'static str {
    match light {
        TraficLight::Red => "Stop",
        TraficLight::Yellow => "Slow down",
        TraficLight::Green => "Go",
    }
}

fn main() {
    let light = TraficLight::Red;
    println!("Red -> {}", action(light));
    let light = TraficLight::Yellow;
    println!("Yellow -> {}", action(light));
    let light = TraficLight::Green;
    println!("Green -> {}", action(light));
}
