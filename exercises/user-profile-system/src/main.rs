
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

impl User {
    fn new(username: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            sign_in_count: 0,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn display_info(&self) {
        println!("username : {} -> email: {} -> sign in count: {} -> is active: {}", self.username, self.email, self.sign_in_count, self.active);
    }
}

fn main() {
    let mut user = User::new("Alice", "alice@email.com");

    user.display_info();

    user.deactivate();

    user.display_info();
}
