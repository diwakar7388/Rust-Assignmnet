
//    Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn main(){

    let sortedArray= vec![1,2,3,4,5,4,5,6,5,6];
    let targetValue=5;
      match firstOccurence(&sortedArray , targetValue){
        Some(indexValue) => println!("First occurrence of value: {}, is at index {}", targetValue, indexValue),
        None => println!("{} Not found!", targetValue),
      }


}


fn firstOccurence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)

}