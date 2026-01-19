use rand::Rng;

fn is_prime(n: u32) -> bool {
    if n <= 1 { return false; }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 { return false; }
    }
    true
}

fn main() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..10000);
    
    if is_prime(number) {
        println!("随机数 {} 是质数！", number);
    } else {
        println!("随机数 {} 不是质数。", number);
    }
}