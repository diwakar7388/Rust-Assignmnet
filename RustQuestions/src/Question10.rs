//    Check if a number is prime in Rust



fn checkPrime(number: u64) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=number / 2 {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}


fn main() {
    let num = 23;
    println!("Is provided number: {} prime? ->  {}", num, checkPrime(num));
}
