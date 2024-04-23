fn main() {
    fizzbuzz(100);
}

fn is_divisible_by(dividend: i32, divisor: i32) -> bool {
    dividend % divisor == 0
}

fn fizzbuzz(n: i32) {
    for i in 1..=n {
        if is_divisible_by(i, 15) {
            println!("FizzBuzz");
        } else if is_divisible_by(i, 3) {
            println!("Fizz");
        } else if is_divisible_by(i, 5) {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_divisible_by() {
        assert_eq!(is_divisible_by(4, 2), true);
        assert_eq!(is_divisible_by(5, 2), false);
    }
}
