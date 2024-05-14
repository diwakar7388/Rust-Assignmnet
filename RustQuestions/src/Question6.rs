//     Implement a function that finds the longest common prefix of a given set of strings.



fn longestCommonPrefix(string: &[String]) -> Option<String> {
    if string.is_empty() {
      return None;
    }
  
    let mut prefix = string[0].clone();
    for str in string.iter().skip(1) {
      while !str.starts_with(&prefix) {
        prefix.truncate(prefix.len() - 1);
        if prefix.is_empty() {
          return None;
        }
      }
    }
    Some(prefix)
  }
  
  fn main() {
    let words = vec![String::from("Ramanagar"), String::from("Raman"), String::from("Ram")];
    let prefix = longestCommonPrefix(&words);
    match prefix {
      Some(pref) => println!("Longest common prefix is here: {}", pref),
      None => println!("No common prefix found"),
    }
  }
  