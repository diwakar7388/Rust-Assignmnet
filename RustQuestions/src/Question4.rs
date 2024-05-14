//     Implement a function that checks whether a given number is prime or not.



fn main() {
    let number = 17;
    println!("Is {} prime? {}", number, checkPrimeNumber(number));
}


fn checkPrimeNumber(number: u64) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=number / 2 {
        if number % i == 0 {
            return false;
        }
    }
    return true
}



