//Function with input u32 and return Option，
fn sum_u32(i: &[u32]) -> Option<u32> {
    i.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
    }
    
fn main() {
    //Create a Vector as container of numbers
    let mut num_series = vec![1, 3, 5, 7, 9, 11];
    
    //Match Pattern calling function sum_u32
    match sum_u32(&num_series) {
        Some(v) => println!("Sum of Series is {}", v),
        None => println!("Overflow Occurs"),
    }
    
    //Match Pattern calling function sum_u32 trying overflow
    let num_series = vec![1, u32::MAX];
    match sum_u32(&num_series) {
        Some(v) => println!("Sum of Series is {}", v),
        None => println!("Overflow Occurs"),
    }
}