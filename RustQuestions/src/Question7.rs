//   Implement a function that returns the kth smallest element in a given array.



fn kthSmallestElement(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted = arr.to_vec();    
        sorted.sort();
        Some(sorted[k - 1])
    } else {
        None
    }
}

fn main() {
    let arr = vec![4, 2, 5, 1, 3];
    let k = 3;
    println!("{} is the  smallest element found here is: {:?}", k, kthSmallestElement(&arr, k));
}
