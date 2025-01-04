use rand::Rng;

pub fn elect(n: usize) -> usize {
    println!("Electing...");
    rand::thread_rng().gen_range(0..n)
}
