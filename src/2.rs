// Rust code for generating a random number between 1 and 10
fn main() {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(1, 11);
    println!("The random number is {}", num);
}
