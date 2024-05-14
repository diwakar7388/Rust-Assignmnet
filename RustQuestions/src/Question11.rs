//      Merge two sorted arrays in Rust





fn mergeSortedArrays(arr1: &[i32], arr2: &[i32])->Vec<i32>    {

    let mut mergedArray = Vec::with_capacity(arr1.len() + arr2.len());

    let mut i= 0;
    let mut j= 0;
  
    while i < arr1.len() && j < arr2.len() {
      if arr1[i] <= arr2[j] {
        mergedArray.push(arr1[i]);
        i += 1;
      } else {
        mergedArray.push(arr2[j]);
        j += 1;
      }
    }
  
 
    mergedArray.extend_from_slice(&arr1[i..]);
    mergedArray.extend_from_slice(&arr2[j..]);
  
    return mergedArray
  }
  
  fn main() {
    let arr1 = vec![5,4,7,8,4];
    let arr2 = vec![2,7,1,2,3,5];
    let mergedResult = mergeSortedArrays(&arr1, &arr2);
    println!("Merged array here is: {:?}", mergedResult);
  }
  