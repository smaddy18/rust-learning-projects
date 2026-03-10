
struct Product {
    name: String,
    price: f64,
    stock: u32,
}

impl Product {
    fn new(name: &str, price: f64, stock: u32) -> Self {
        Self {
            name: name.to_string(),
            price,
            stock,
        }
    }

    fn purchase(&mut self, quantity: u32) {
        if quantity > self.stock {
            println!("There is not enough items in stock !");
            return;
        }
        self.stock -= quantity;
        println!("Successfully purchased -> now the stock is {}", self.stock);
    }

    fn restock(&mut self, quantity: u32) {
        self.stock += quantity;
    }

    fn total_value(&self) -> f64 {
        (self.stock as f64) * self.price
    }
}

fn main() {
    let mut product = Product::new("Keyboard", 50.0, 10);

    println!("Total value : {}", product.total_value());

    product.purchase(12);

    product.restock(5);
    println!("Total value : {}", product.total_value());

    product.purchase(12);

    println!("Total value : {}", product.total_value());
}
