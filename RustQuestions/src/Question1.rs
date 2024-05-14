//     Implement a function that checks whether a given string is a palindrome or not.


fn main() {
    let s = "radar";
    println!("Is provided string a palindrome? ->{}",  checkPalindrome(s));
}


fn checkPalindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
