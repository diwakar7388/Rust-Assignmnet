//      Given a sorted array of integers, implement a function that returns the median of the array.




fn main() {
    let number = vec![1,2,3,4,5];
    
    let median = findMedian(&number);
  
   match median {
        Some(value) => println!("Median of {:?}: {}", number, value),
        None => println!("Array is empty!"),
    }

}



fn findMedian(arr: &[i32]) -> Option<f64> {
    let length = arr.len();
    if length == 0 {
        return None;
    }

    let middleIndex = length / 2;

    if length % 2 == 0 {
       
        let median = (arr[middleIndex - 1] as f64 + arr[middleIndex] as f64) / 2.0;
        return Some(median);
    } else {
       
        return Some(arr[middleIndex] as f64);
    }
}

