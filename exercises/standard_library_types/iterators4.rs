// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    match num {
        0 | 1 => 1,
        n => (1_u64..num+1).fold(1, |fact, next| fact * next),

        // alternatively:
        // n => (1..num+1).fold::<u64, _>(1, |fact, next| fact * next),        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn factorial_of_big_number() {
        assert_eq!(2432902008176640000, factorial(20));
    }
}
