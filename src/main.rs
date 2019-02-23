//This is an attempt at completing the list string problems from https://adriann.github.io/programming_problems.html

fn main() {
    println!("Hello, world!");
    let numbers = vec![12, 10, 6, -4, 3, 42];
    println!("{}", largest_element(&numbers));
    println!("{}", smallest_element(&numbers));
}

// find the largest element in a vector
fn largest_element(nums: &Vec<i64>) -> i64 {
    let mut largest: i64 = std::i64::MIN;
    for i in nums.iter() {
        if largest < *i {
            largest = *i;
        }
    }
    largest
}

fn smallest_element(nums: &Vec<i64>) -> i64 {
    let mut smallest: i64 = std::i64::MAX;
    for i in nums.iter() {
        if smallest > *i {
            smallest = *i;
        }
    }
    smallest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_element() {
        assert_eq!(smallest_element(&vec![12, 10, 6, -4, 3, 42]), -4);
        assert_eq!(smallest_element(&vec![1, 10, 6, 40, 3, 42]), 1);
    }

    #[test]
    fn test_largest_element() {
        assert_eq!(largest_element(&vec![12, 10, 6, -4, 3, 42]), 42);
        assert_eq!(largest_element(&vec![-1, -10, -6, -40, -3, -42]), -1);
    }
}
