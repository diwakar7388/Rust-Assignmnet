//      Find the maximum subarray sum in Rust




fn maximumSubarraySum(arr: &[i32]) -> i32 {
    let mut maxSum = arr[0];
    let mut currentSum = arr[0];
    for &num in arr.iter().skip(1) {
        currentSum = currentSum.max(num);
        maxSum = maxSum.max(currentSum);
    }
     return maxSum
}

fn main() {
    let arr = vec![-4,3,4,-5,7,-8,6,1];
    println!("Maximum subarray sum here found is : {}", maximumSubarraySum(&arr));
}
