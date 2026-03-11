
mod math {
    pub mod geometry {
        use std::f64::consts::PI;

        pub fn area_circle(radius: u32) -> f64 {
            (PI * (radius as f64).powi(2)).round()
        }

        pub fn area_rectangle(width: u32, height: u32) -> u32 {
            width * height
        }
    }

    pub mod algebra {
        pub fn square(num: i32) -> i32 {
            num.pow(2)
        }

        pub fn cube(num: i32) -> i32 {
            num.pow(3)
        }
    }
}

use crate::math::{geometry, algebra};

fn main() {
    println!("Area Circle: {}", geometry::area_circle(2));
    println!("Area Rectangle: {}", geometry::area_rectangle(9, 6));
    println!("Square: {}", algebra::square(2));
    println!("Cube: {}", algebra::cube(3));
}
