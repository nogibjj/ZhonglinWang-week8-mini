use rand::seq::SliceRandom;
// use rand::Rng;

const FRUITS: &[&str] = &[
    "Apple",
    "Banana",
    "Orange",
    "Pineapple",
    "Strawberry",
    "Mango",
    "Grapes",
    "Kiwi",
    "Watermelon",
    "Peach",
];

fn main() {
    let mut rng = rand::thread_rng();
    let fruit = FRUITS.choose(&mut rng).unwrap();
    println!("{}", fruit);
}
