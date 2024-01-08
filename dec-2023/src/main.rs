/**
 * Rust Program to find the smallest integer value that is missing in the array.
 */
use core::cmp::Ordering;

pub fn solution(a_array: &mut Vec<u32>) -> u32 {
    a_array.sort();
    if let Some(pos) = a_array.iter().position(|x| *x <= 0) {
        a_array.remove(pos);
    }
    let mut smallest: u32 = 1;
    for element in a_array {
        // println!("value of smallest variable is: {smallest} and current array element value is: {element}");
        match smallest.cmp(&element) {
            Ordering::Less => {
                // println!("Finished");
                break;
            }
            Ordering::Equal | std::cmp::Ordering::Greater => {
                // println!("Lets Increment");
                smallest += 1;
                continue;
            }
        }
    }
    // println!("Smallest value that is missing in the array is {smallest}");
    println!("Answer is: {smallest}");
    smallest
}

fn main(){
    solution(&mut vec![3, 8, 0, 5]);
    solution(&mut vec![3, 8, 1, 5]);
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn find_smallest_fail_1() {
        assert_eq!(solution(&mut vec![1, 3, 5, 8]), 2);
    }
    #[test]
    fn find_smallest_pass() {
        assert_eq!(solution(&mut vec![0, 3, 5, 8]), 1);
    }
    #[test]
    fn find_smallest_should_pass() {
        assert_eq!(solution(&mut vec![50, 82]), 1);
    }
}