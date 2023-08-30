
fn main() {
    print_name("Patrique");
}

fn print_name(name: &str) {
    if !name.is_empty() {
        println!("Hello world, {}!", name);
    } else {
        println!("Hello, world!");
    }
}

