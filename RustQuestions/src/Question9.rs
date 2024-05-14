//     Reverse a string in Rust




fn reverseString(text: &str) -> String {
    let mut reversedString = String::new();
    for char in text.chars().rev() {
      reversedString.push(char);
    }
    reversedString
  }
  
  fn main() {
   
   let givenString= "Hello , how are you";
    let reversedOne = reverseString(givenString);
    println!("Reversed string obtained is : {}", reversedOne);
  }
  
