use std::io;

fn main() {
    let x = get_input("Enter x: ");
    let y = get_input("Enter y: ");
    let z = get_input("Enter z: ");

    let coordinates = (x, y, z);

    println!("Coordinates: {:?}", coordinates);
}

fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, please enter a number"),
        }
    }
}
