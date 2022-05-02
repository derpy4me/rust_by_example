pub fn main() {
    // {:b} changes input to binary
    println!("{} of {:b} people know binary.", 1, 2);

    // Justifies text by adding x spaces before
    println!("{number:>width$}", number = 1, width = 6);

    // You can change from spaces to any character
    println!("{number:q>width$}", number = 1, width = 6);

    // Go the other way?
    println!("{number:q<width$}", number = 1, width = 6);

    // Center aligned
    println!("{number:q^width$}", number = 1, width = 7);

    print_pi_rounded();

    let me = SomeStuff {
        name: String::from("my name"),
        age: 98,
    };

    println!("Something about me: {me:?}");
}

fn print_pi_rounded() {
    // https://doc.rust-lang.org/std/fmt/
    let pi = 3.141592;
    println!("Pi rounded: {pi:.3}")
}

#[derive(Debug)]
#[allow(dead_code)]
struct SomeStuff {
    name: String,
    age: u16,
}
