//This is an attempt at completing the list string problems from https://adriann.github.io/programming_problems.html

fn main() {
    println!("Hello, world!");
    let mut numbers = vec![12, 10, 6, -4, 3, 42];
    let mut numbers2 = vec![2, 0, 16, -14, 13, 2];
    println!("{}", largest_element(&numbers));
    println!("{}", smallest_element(&numbers));
    numbers = reverse_inplace(numbers);
    println!("{:?}", numbers);

    println!("{}", occurs_in_vector(&numbers, 42));
    println!("{:?}", odd_positions(&numbers));
    println!("{}", running_total(&numbers));
    let stringy = String::from("Hannah");
    println!("{}", palindrone(&stringy));
    let stringy2 = String::from("HannaH");
    println!("{}", palindrone(&stringy2));

    println!("{}", while_loop(&numbers));

    println!("{:?}", concat_vectors(&numbers, &numbers2));
    println!("{:?}", mixed_concat_vectors(&numbers, &numbers2));

    //merge sorted vectors
    let num1 = vec![0, 1, 3, 8, 10, 12];
    let num2 = vec![2, 5, 8, 11, 13, 21];
    println!("{:?}", merge_sort_vectors(&num1, &num2));

    //Rotate cheating by using the builtin one
    let mut num3 = vec![1, 2, 3, 4, 5, 6];
    num3 = rotate_cheating(num3, 2);
    println!("{:?}", num3);

    //number_to_digits
    let realnumber: i64 = 123456789;
    let stringnumber = number_to_digits(realnumber);
    println!("{:?}", stringnumber);
}

// 1) Write a function that returns the largest element in a list.
fn largest_element(nums: &Vec<i64>) -> i64 {
    let mut largest: i64 = std::i64::MIN;
    for i in nums.iter() {
        if largest < *i {
            largest = *i;
        }
    }
    largest
}

// 1a) just for fun the opposite of 1
fn smallest_element(nums: &Vec<i64>) -> i64 {
    let mut smallest: i64 = std::i64::MAX;
    for i in nums.iter() {
        if smallest > *i {
            smallest = *i;
        }
    }
    smallest
}

// 2) Write function that reverses a list, preferably in place
fn reverse_inplace(mut nums: Vec<i64>) -> Vec<i64> {
    nums.reverse();
    return nums;
}

//3) Write a function that checks whether an element occurs in a list.
fn occurs_in_vector(nums: &Vec<i64>, number: i64) -> bool {
    nums.contains(&number)
}

//4) Write a function that returns the elements on odd positions in a list.
fn odd_positions(nums: &Vec<i64>) -> Vec<i64> {
    let mut odds: Vec<i64> = Vec::new();
    for (k, v) in nums.iter().enumerate() {
        if k % 2 != 0 {
            odds.push(*v);
        }
    }
    odds
}

// 5) Write a function that computes the running total of a list.
fn running_total(nums: &Vec<i64>) -> i64 {
    let mut total: i64 = 0;
    for v in nums.iter() {
        total = total + *v;
    }
    total
}

// 6) Write a function that tests whether a string is a palindrome.
fn palindrone(input_string: &String) -> bool {
    let mut rev_string = String::from("");

    for v in input_string.chars().rev() {
        rev_string.push(v)
    }

    if rev_string == *input_string {
        return true;
    }
    false
}

// 7) Write three functions that compute the sum of the numbers in a list: using a for-loop, a while-loop and recursion. (Subject to availability of these constructs in your language of choice.)

fn while_loop(input: &Vec<i64>) -> i64 {
    let mut done: bool = false;
    let mut total: i64 = 0;
    let mut counter = input.len() - 1;

    while !done {
        total = total + input[counter];
        if counter == 0 {
            done = true;
            continue;
        }
        counter -= 1;
    }
    total
}

// 8)

// 9) Write a function that concatenates two lists. [a,b,c], [1,2,3] → [a,b,c,1,2,3]

fn concat_vectors(input1: &Vec<i64>, input2: &Vec<i64>) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();

    for i in input1.iter() {
        output.push(*i);
    }

    for i in input2.iter() {
        output.push(*i);
    }
    output
}

// 10) Write a function that combines two lists by alternatingly taking elements, e.g. [a,b,c], [1,2,3] → [a,1,b,2,c,3].

fn mixed_concat_vectors(input1: &Vec<i64>, input2: &Vec<i64>) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();

    let mut maxsize = input1.len();
    if maxsize < input2.len() {
        maxsize = input2.len();
    }

    for i in 0..maxsize {
        if i < input1.len() {
            output.push(input1[i]);
        }
        if i < input2.len() {
            output.push(input2[i]);
        }
    }

    output
}

// 11) Write a function that merges two sorted lists into a new sorted list. [1,4,6],[2,3,5] → [1,2,3,4,5,6]. You can do this quicker than concatenating them followed by a sort.

fn merge_sort_vectors(input1: &Vec<i64>, input2: &Vec<i64>) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();

    let maxsize = input1.len() + input2.len();

    let mut pos1: usize = 0;
    let mut pos2: usize = 0;

    for _i in 0..maxsize {
        let temp = if pos1 < input1.len() {
            input1[pos1]
        } else {
            std::i64::MAX
        };
        if pos2 < input2.len() {
            if input2[pos2] < temp {
                output.push(input2[pos2]);
                pos2 += 1;
            } else {
                output.push(temp);
                pos1 += 1;
            }
        }
    }
    output
}

// 12) Write a function that rotates a list by k elements. For example [1,2,3,4,5,6] rotated by two becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?

fn rotate_cheating(mut input1: Vec<i64>, left: usize) -> Vec<i64> {
    input1.rotate_left(left);
    input1
}

// 13)

// 14) Write a function that takes a number and returns a list of its digits. So for 2342 it should return [2,3,4,2].

fn number_to_digits(input: i64) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    let s: String = input.to_string();

    for i in s.chars() {
        output.push(i64::from(i.to_digit(10).unwrap()));
    }

    output
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
