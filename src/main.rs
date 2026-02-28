fn main() {
    println!("Some fib numbers:");
    println!("{}", fib(-3));
    println!("{}", fib(1));
    println!("{}", fib(2));
    println!("{}", fib(3));
    println!("{}", fib(4));
    println!("{}", fib(5));

    println!("Some luc numbers:");
    println!("{}", luc(0));
    println!("{}", luc(1));
    println!("{}", luc(2));
    println!("{}", luc(3));
    println!("{}", luc(4));
    println!("{}", luc(5));
}

fn fib(n: i32) -> i32 {
    if n < 0 {
        return 100;
    }
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn luc(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 2;
    } else if n == 1 {
        return 1;
    }
    luc(n - 1) + luc(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*; 


    #[test]
    fn test_fib_positive_cases() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
    }
    #[test]
    fn test_luc_positive_cases() {
        assert_eq!(luc(0), 2);
        assert_eq!(luc(1), 1);
        assert_eq!(luc(2), 3);
        assert_eq!(luc(3), 4);
        assert_eq!(luc(4), 7);
        assert_eq!(luc(5), 11);
    }
    #[test]
    fn test_fib_false_cases() {
        assert_ne!(fib(5), 10);
        assert_ne!(fib(0), 1);
    }
    #[test]
    fn test_fib_rejects_negative_numbers() {
        assert_eq!(fib(-3), 0);
    }
    #[test]
    fn test_luc_rejects_negative_numbers() {
        assert_eq!(luc(-3), 0);
    }
    #[test]
    fn test_fib_big_cases() {
        assert_eq!(fib(20), 6765);
    }
}   