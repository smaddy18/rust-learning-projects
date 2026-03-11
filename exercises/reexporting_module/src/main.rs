use reexporting_module::company::{self, management};

fn main() {
    company::add_employees("John");
    company::list_employees();

    management::promote_employee("John");
}