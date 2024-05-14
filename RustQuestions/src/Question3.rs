//     Given a string of words, implement a function that returns the shortest word in the string.


fn shortestWordInString(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|w| w.len())
}


fn main() {
    let s = "My name is Diwakar Dixit";
    println!("Shortest word in a give string : {:?}", shortestWordInString(s));
}
