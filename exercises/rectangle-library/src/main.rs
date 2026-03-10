
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    let r1 = Rectangle {
        width: 9,
        height: 6
    };
    let r2 = Rectangle {
        width: 7,
        height: 3
    };

    let area1 = r1.area();
    let area2 = r2.area();

    let perimeter1 = r1.perimeter();
    let perimeter2 = r2.perimeter();

    let can_hold_r2 = r1.can_hold(&r2);

    println!("Areas -> r1 : {} and r2 : {}", area1, area2);
    println!("Perimeters -> r1 : {} and r2 : {}", perimeter1, perimeter2);
    println!("r1 can hold r2 ? {}", can_hold_r2);
}
